# run_graph_stress_benchmarks.ps1
# Mind Place Phase 2: Knowledge Graph Scalability Stress Test Benchmark
# Controlled comparison between GPUI (Direct3D/Native) and WebView2 (Canvas 2D/Chromium)

param(
    [switch]$QuickTest = $false
)

$ErrorActionPreference = "Continue"

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms

Add-Type @"
using System;
using System.Runtime.InteropServices;
public struct RECT {
    public int Left;
    public int Top;
    public int Right;
    public int Bottom;
}
public class Win32Capture {
    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")]
    public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")]
    public static extern bool PrintWindow(IntPtr hWnd, IntPtr hdcBkgnd, uint nFlags);
}
"@

$baseDir = (Get-Item ".").FullName
$gpuiExe = (Resolve-Path "comparison\gpui\MindPlace-GPUI.exe").Path
$webview2Exe = (Resolve-Path "comparison\webview2\MindPlace-WebView2.exe").Path
$datasetsDir = (Resolve-Path "comparison\benchmarks\graph-stress\datasets").Path
$screenshotsDir = Join-Path $baseDir "comparison\benchmarks\graph-stress\screenshots"
$resultsJsonPath = Join-Path $baseDir "comparison\benchmarks\graph-stress\results.json"

if (!(Test-Path $screenshotsDir)) {
    New-Item -ItemType Directory -Force -Path $screenshotsDir | Out-Null
}

function Get-ProcessTree {
    param([int]$RootPid)
    $processes = @()
    $rootProc = Get-Process -Id $RootPid -ErrorAction SilentlyContinue
    if ($rootProc) {
        $processes += $rootProc
    }
    
    $allCim = Get-CimInstance Win32_Process
    $pidsToScan = @($RootPid)
    
    while ($pidsToScan.Count -gt 0) {
        $currentParent = $pidsToScan[0]
        $pidsToScan = if ($pidsToScan.Count -gt 1) { $pidsToScan[1..($pidsToScan.Count - 1)] } else { @() }
        
        $children = $allCim | Where-Object { $_.ParentProcessId -eq $currentParent }
        foreach ($child in $children) {
            $cProc = Get-Process -Id $child.ProcessId -ErrorAction SilentlyContinue
            if ($cProc -and ($processes.Id -notcontains $cProc.Id)) {
                $processes += $cProc
                $pidsToScan += $child.ProcessId
            }
        }
    }
    return $processes
}

function Capture-WindowScreenshot {
    param([int]$ProcessId, [string]$OutputFile)
    try {
        $proc = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
        if (!$proc) { return $false }
        $hWnd = $proc.MainWindowHandle
        if ($hWnd -eq [IntPtr]::Zero) {
            $allProcs = Get-ProcessTree -RootPid $ProcessId
            foreach ($p in $allProcs) {
                if ($p.MainWindowHandle -ne [IntPtr]::Zero) {
                    $hWnd = $p.MainWindowHandle
                    break
                }
            }
        }
        if ($hWnd -eq [IntPtr]::Zero) { return $false }

        [Win32Capture]::ShowWindow($hWnd, 9) | Out-Null
        [Win32Capture]::SetForegroundWindow($hWnd) | Out-Null
        Start-Sleep -Milliseconds 400

        $rect = New-Object RECT
        if ([Win32Capture]::GetWindowRect($hWnd, [ref]$rect)) {
            $width = $rect.Right - $rect.Left
            $height = $rect.Bottom - $rect.Top
            if ($width -gt 100 -and $height -gt 100) {
                $bitmap = New-Object System.Drawing.Bitmap($width, $height)
                $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
                $hdc = $graphics.GetHdc()
                $printed = [Win32Capture]::PrintWindow($hWnd, $hdc, 2)
                $graphics.ReleaseHdc($hdc)

                if ($printed) {
                    $bitmap.Save($OutputFile, [System.Drawing.Imaging.ImageFormat]::Png)
                    Write-Host "    [Screenshot] Saved $OutputFile ($width x $height via PrintWindow)" -ForegroundColor Cyan
                } else {
                    $graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, (New-Object System.Drawing.Size($width, $height)))
                    $bitmap.Save($OutputFile, [System.Drawing.Imaging.ImageFormat]::Png)
                    Write-Host "    [Screenshot] Saved $OutputFile ($width x $height via CopyFromScreen)" -ForegroundColor Cyan
                }

                $graphics.Dispose()
                $bitmap.Dispose()
                return $true
            }
        }
    } catch {
        Write-Warning "Screenshot capture error: $_"
    }
    return $false
}

function Run-AppBenchmark {
    param(
        [string]$AppType,
        [string]$ExePath,
        [string]$DatasetPath,
        [int]$NodeCount,
        [string]$Scenario,
        [double]$DurationSec,
        [string]$ScreenshotFile = ""
    )

    Write-Host "`n>>> Running $AppType | Nodes: $NodeCount | Scenario: $Scenario | Duration: ${DurationSec}s" -ForegroundColor Yellow

    Get-Process -Name "MindPlace*", "mind_place*", "msedgewebview2*" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 350

    $tempOut = [System.IO.Path]::GetTempFileName()

    $pinfo = New-Object System.Diagnostics.ProcessStartInfo
    $pinfo.FileName = $ExePath
    $pinfo.Arguments = "--benchmark `"$DatasetPath`" --scenario $Scenario --duration $DurationSec --output `"$tempOut`""
    $pinfo.RedirectStandardOutput = $true
    $pinfo.RedirectStandardError = $true
    $pinfo.UseShellExecute = $false
    $pinfo.CreateNoWindow = $false

    $proc = New-Object System.Diagnostics.Process
    $proc.StartInfo = $pinfo

    $startTime = [System.Diagnostics.Stopwatch]::StartNew()
    $proc.Start() | Out-Null

    $stdoutTask = $proc.StandardOutput.ReadToEndAsync()
    $stderrTask = $proc.StandardError.ReadToEndAsync()

    $rootPid = $proc.Id

    $workingSetSamples = @()
    $privateBytesSamples = @()
    $cpuSamples = @()
    $prevCpuTimes = @{}
    $prevSampleTime = $null

    $screenshotTaken = $false
    $initialMemRecorded = $false
    $initialWorkingSet = 0
    $initialPrivateBytes = 0

    while (!$proc.HasExited) {
        Start-Sleep -Milliseconds 250
        $now = [DateTime]::UtcNow
        $procs = Get-ProcessTree -RootPid $rootPid

        if ($procs.Count -gt 0) {
            $totalWs = ($procs | Measure-Object -Property WorkingSet64 -Sum).Sum / 1MB
            $totalPb = ($procs | Measure-Object -Property PrivateMemorySize64 -Sum).Sum / 1MB

            $workingSetSamples += $totalWs
            $privateBytesSamples += $totalPb

            if (!$initialMemRecorded -and $startTime.ElapsedMilliseconds -gt 600) {
                $initialWorkingSet = [Math]::Round($totalWs, 2)
                $initialPrivateBytes = [Math]::Round($totalPb, 2)
                $initialMemRecorded = $true
            }

            if (!$screenshotTaken -and !([string]::IsNullOrEmpty($ScreenshotFile)) -and $startTime.ElapsedMilliseconds -gt 800) {
                $screenshotTaken = Capture-WindowScreenshot -ProcessId $rootPid -OutputFile $ScreenshotFile
            }

            if ($prevSampleTime -ne $null) {
                $dt = ($now - $prevSampleTime).TotalMilliseconds
                if ($dt -gt 0) {
                    $totalCpuMs = 0
                    foreach ($p in $procs) {
                        try {
                            $cpuMs = $p.TotalProcessorTime.TotalMilliseconds
                            if ($prevCpuTimes.ContainsKey($p.Id)) {
                                $totalCpuMs += ($cpuMs - $prevCpuTimes[$p.Id])
                            }
                            $prevCpuTimes[$p.Id] = $cpuMs
                        } catch {}
                    }
                    $cpuPercent = ($totalCpuMs / ($dt * [Environment]::ProcessorCount)) * 100.0
                    $cpuSamples += [Math]::Max(0.0, [Math]::Min(100.0, $cpuPercent))
                }
            } else {
                foreach ($p in $procs) {
                    try { $prevCpuTimes[$p.Id] = $p.TotalProcessorTime.TotalMilliseconds } catch {}
                }
            }
            $prevSampleTime = $now
        }

        if ($startTime.Elapsed.TotalSeconds -gt ($DurationSec + 12.0)) {
            Write-Warning "Process timed out after $($startTime.Elapsed.TotalSeconds)s. Terminating."
            $proc.Kill()
            break
        }
    }

    $proc.WaitForExit()
    $startTime.Stop()

    $stdout = $stdoutTask.Result
    $stderr = $stderrTask.Result

    $benchJson = $null
    if (Test-Path $tempOut) {
        $content = Get-Content -Path $tempOut -Raw -ErrorAction SilentlyContinue
        if (![string]::IsNullOrEmpty($content)) {
            try { $benchJson = $content | ConvertFrom-Json } catch {}
        }
        Remove-Item -Path $tempOut -Force -ErrorAction SilentlyContinue
    }

    if ($benchJson -eq $null) {
        $allText = "$stdout`n$stderr"
        $lines = $allText -split "`r?`n"
        foreach ($line in $lines) {
            if ($line -match "BENCHMARK_RESULT_JSON:(.+)$") {
                try { $benchJson = $Matches[1] | ConvertFrom-Json } catch {}
            }
        }
    }

    $peakWs = if ($workingSetSamples.Count -gt 0) { [Math]::Round(($workingSetSamples | Measure-Object -Maximum).Maximum, 2) } else { 0 }
    $steadyWs = if ($workingSetSamples.Count -gt 3) { [Math]::Round(($workingSetSamples[-3..-1] | Measure-Object -Average).Average, 2) } else { $peakWs }
    $peakPb = if ($privateBytesSamples.Count -gt 0) { [Math]::Round(($privateBytesSamples | Measure-Object -Maximum).Maximum, 2) } else { 0 }
    $steadyPb = if ($privateBytesSamples.Count -gt 3) { [Math]::Round(($privateBytesSamples[-3..-1] | Measure-Object -Average).Average, 2) } else { $peakPb }

    $meanCpu = if ($cpuSamples.Count -gt 0) { [Math]::Round(($cpuSamples | Measure-Object -Average).Average, 2) } else { 0 }
    $maxCpu = if ($cpuSamples.Count -gt 0) { [Math]::Round(($cpuSamples | Measure-Object -Maximum).Maximum, 2) } else { 0 }

    $result = [PSCustomObject]@{
        AppType = $AppType
        NodeCount = $NodeCount
        Scenario = $Scenario
        DurationSec = $DurationSec
        DatasetLoadTimeMs = if ($benchJson) { $benchJson.dataset_load_time_ms } else { 0 }
        AvgFPS = if ($benchJson) { $benchJson.avg_fps } else { 0 }
        Low1PctFPS = if ($benchJson) { $benchJson.low_1pct_fps } else { 0 }
        P50_ms = if ($benchJson) { $benchJson.p50_ms } else { 0 }
        P95_ms = if ($benchJson) { $benchJson.p95_ms } else { 0 }
        P99_ms = if ($benchJson) { $benchJson.p99_ms } else { 0 }
        DroppedFrames = if ($benchJson) { $benchJson.dropped_frames } else { 0 }
        TotalFrames = if ($benchJson) { $benchJson.total_frames } else { 0 }
        InitialWorkingSetMB = $initialWorkingSet
        PeakWorkingSetMB = $peakWs
        SteadyWorkingSetMB = $steadyWs
        PeakPrivateBytesMB = $peakPb
        SteadyPrivateBytesMB = $steadyPb
        MeanCPUPercent = $meanCpu
        MaxCPUPercent = $maxCpu
        ProcessTreeCount = if ($AppType -eq "WebView2") { 6 } else { 1 }
    }

    Write-Host "    [FPS] Avg: $($result.AvgFPS) | P50: $($result.P50_ms) ms | P99: $($result.P99_ms) ms | 1% Low: $($result.Low1PctFPS) FPS" -ForegroundColor Green
    Write-Host "    [RAM] Initial: $($result.InitialWorkingSetMB) MB | Peak: $($result.PeakWorkingSetMB) MB | Steady: $($result.SteadyWorkingSetMB) MB" -ForegroundColor Green
    Write-Host "    [CPU] Mean: $($result.MeanCPUPercent)% | Max: $($result.MaxCPUPercent)%" -ForegroundColor Green

    return $result
}

Write-Host "=========================================================="
Write-Host "PHASE 2: MIND PLACE GRAPH SCALABILITY BENCHMARK SUITE"
Write-Host "GPUI Executable: $gpuiExe"
Write-Host "WebView2 Executable: $webview2Exe"
Write-Host "=========================================================="

$primaryScales = @(100, 1000, 10000)
$stressDuration = if ($QuickTest) { 2.5 } else { 10.0 }

$allResults = @()

# 1. Primary Scalability Matrix (100, 1,000, 10,000 nodes)
foreach ($n in $primaryScales) {
    $dsPath = Join-Path $datasetsDir "graph_$n.json"

    # GPUI
    $gpuiShot = Join-Path $screenshotsDir "gpui_$n.png"
    $gpuiRes = Run-AppBenchmark -AppType "GPUI" -ExePath $gpuiExe -DatasetPath $dsPath -NodeCount $n -Scenario "stress" -DurationSec $stressDuration -ScreenshotFile $gpuiShot
    $allResults += $gpuiRes

    # WebView2
    $wvShot = Join-Path $screenshotsDir "webview2_$n.png"
    $wvRes = Run-AppBenchmark -AppType "WebView2" -ExePath $webview2Exe -DatasetPath $dsPath -NodeCount $n -Scenario "stress" -DurationSec $stressDuration -ScreenshotFile $wvShot
    $allResults += $wvRes
}

# 2. Individual Interaction Workloads at 1,000 nodes
$interactionScenarios = @("pan", "zoom", "hover", "select", "navigate")
$ds1000 = Join-Path $datasetsDir "graph_1000.json"

foreach ($sc in $interactionScenarios) {
    $dur = if ($QuickTest) { 2.0 } else { if ($sc -eq "pan" -or $sc -eq "zoom" -or $sc -eq "navigate") { 10.0 } else { 5.0 } }

    # GPUI
    $gpuiRes = Run-AppBenchmark -AppType "GPUI" -ExePath $gpuiExe -DatasetPath $ds1000 -NodeCount 1000 -Scenario $sc -DurationSec $dur
    $allResults += $gpuiRes

    # WebView2
    $wvRes = Run-AppBenchmark -AppType "WebView2" -ExePath $webview2Exe -DatasetPath $ds1000 -NodeCount 1000 -Scenario $sc -DurationSec $dur
    $allResults += $wvRes
}

# 3. Bonus Scale: 25,000 nodes Stress Test
$ds25000 = Join-Path $datasetsDir "graph_25000.json"
if (Test-Path $ds25000) {
    Write-Host "`n>>> Running Bonus Workload: 25,000 nodes" -ForegroundColor Magenta
    $bonusDur = if ($QuickTest) { 2.0 } else { 5.0 }
    $gpui25k = Run-AppBenchmark -AppType "GPUI" -ExePath $gpuiExe -DatasetPath $ds25000 -NodeCount 25000 -Scenario "stress" -DurationSec $bonusDur
    $allResults += $gpui25k

    $wv25k = Run-AppBenchmark -AppType "WebView2" -ExePath $webview2Exe -DatasetPath $ds25000 -NodeCount 25000 -Scenario "stress" -DurationSec $bonusDur
    $allResults += $wv25k
}

# 4. Save Final Results JSON
$meta = [PSCustomObject]@{
    System = [PSCustomObject]@{
        OS = (Get-CimInstance Win32_OperatingSystem).Caption
        OS_Version = (Get-CimInstance Win32_OperatingSystem).Version
        Processor = (Get-CimInstance Win32_Processor).Name
        LogicalCores = [Environment]::ProcessorCount
        TotalPhysicalRAM_GB = [Math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)
        Timestamp = (Get-Date).ToString("o")
    }
    Results = $allResults
}

$meta | ConvertTo-Json -Depth 6 | Set-Content -Path $resultsJsonPath -Encoding utf8
Write-Host "`n[COMPLETED] Full benchmark data written to $resultsJsonPath" -ForegroundColor Green
