# benchmark_suite.ps1
# Comprehensive Performance Benchmark for Mind Place: GPUI vs. WebView2

$ErrorActionPreference = "Continue"

$outputDir = "comparison\benchmarks"
if (!(Test-Path $outputDir)) {
    New-Item -ItemType Directory -Force -Path $outputDir | Out-Null
}

$gpuiExe = (Resolve-Path "comparison\gpui\MindPlace-GPUI.exe").Path
$webview2Exe = (Resolve-Path "comparison\webview2\MindPlace-WebView2.exe").Path

Write-Host "=========================================================="
Write-Host "MIND PLACE DESKTOP BENCHMARK: GPUI VS. WEBVIEW2"
Write-Host "GPUI Binary: $gpuiExe"
Write-Host "WebView2 Binary: $webview2Exe"
Write-Host "=========================================================="

# 1. Executable & Runtime Footprint
$gpuiFileInfo = Get-Item $gpuiExe
$webview2FileInfo = Get-Item $webview2Exe

$edgeWebViewRuntimeDir = "C:\Program Files (x86)\Microsoft\EdgeWebView\Application"
$runtimeSizeMB = 0
if (Test-Path $edgeWebViewRuntimeDir) {
    $runtimeFiles = Get-ChildItem -Path $edgeWebViewRuntimeDir -Recurse -File -ErrorAction SilentlyContinue
    $runtimeBytes = ($runtimeFiles | Measure-Object -Property Length -Sum).Sum
    $runtimeSizeMB = [Math]::Round($runtimeBytes / 1MB, 2)
}

$footprint = [PSCustomObject]@{
    GPUI_Exe_Bytes = $gpuiFileInfo.Length
    GPUI_Exe_MB = [Math]::Round($gpuiFileInfo.Length / 1MB, 2)
    GPUI_Runtime_Dependency = "None (Fully self-contained native PE64 executable)"
    GPUI_Total_Disk_MB = [Math]::Round($gpuiFileInfo.Length / 1MB, 2)
    
    WebView2_Exe_Bytes = $webview2FileInfo.Length
    WebView2_Exe_MB = [Math]::Round($webview2FileInfo.Length / 1MB, 2)
    WebView2_Runtime_Dependency = "Microsoft Edge WebView2 Evergreen Runtime"
    WebView2_Runtime_Disk_MB = $runtimeSizeMB
    WebView2_Total_Disk_MB = [Math]::Round(($webview2FileInfo.Length / 1MB) + $runtimeSizeMB, 2)
}

Write-Host "Disk Footprint:"
Write-Host "  GPUI Exe: $($footprint.GPUI_Exe_MB) MB (0 shared dependencies)"
Write-Host "  WebView2 Exe: $($footprint.WebView2_Exe_MB) MB (+ $runtimeSizeMB MB Edge WebView2 Runtime)"

# Helper functions to gather process trees
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
        $currentPid = $pidsToScan[0]
        $pidsToScan = $pidsToScan[1..($pidsToScan.Count)]
        
        $children = $allCim | Where-Object { $_.ParentProcessId -eq $currentPid }
        foreach ($child in $children) {
            $childProc = Get-Process -Id $child.ProcessId -ErrorAction SilentlyContinue
            if ($childProc -and ($processes.Id -notcontains $childProc.Id)) {
                $processes += $childProc
                $pidsToScan += $child.ProcessId
            }
        }
    }
    return $processes
}

function Measure-ProcessTreeMemory {
    param([int]$RootPid)
    $tree = Get-ProcessTree -RootPid $RootPid
    $totalWS = 0
    $totalPrivate = 0
    $details = @()
    
    foreach ($p in $tree) {
        try {
            $p.Refresh()
            $ws = $p.WorkingSet64
            $pm = $p.PrivateMemorySize64
            $totalWS += $ws
            $totalPrivate += $pm
            $details += [PSCustomObject]@{
                Id = $p.Id
                Name = $p.ProcessName
                WorkingSet_MB = [Math]::Round($ws / 1MB, 2)
                PrivateBytes_MB = [Math]::Round($pm / 1MB, 2)
            }
        } catch {}
    }
    
    return [PSCustomObject]@{
        ProcessCount = $tree.Count
        Total_WorkingSet_MB = [Math]::Round($totalWS / 1MB, 2)
        Total_PrivateBytes_MB = [Math]::Round($totalPrivate / 1MB, 2)
        Details = $details
    }
}

# 2. Startup Time Benchmark (10 iterations each)
Write-Host "`nMeasuring Startup Times (10 iterations each)..."

function Benchmark-Startup {
    param([string]$ExePath, [string]$ProcessName)
    $results = @()
    
    for ($i = 1; $i -le 10; $i++) {
        # Ensure clean environment
        Get-Process -Name $ProcessName, "msedgewebview2" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
        Start-Sleep -Milliseconds 500
        
        $sw = [System.Diagnostics.Stopwatch]::StartNew()
        $proc = Start-Process -FilePath $ExePath -PassThru
        
        # Wait until MainWindowHandle is populated and responding
        $ready = $false
        while ($sw.ElapsedMilliseconds -lt 15000) {
            Start-Sleep -Milliseconds 20
            try {
                $proc.Refresh()
                if ($proc.MainWindowHandle -ne [IntPtr]::Zero -and $proc.Responding) {
                    $ready = $true
                    break
                }
            } catch {
                break
            }
        }
        $sw.Stop()
        $elapsedMs = $sw.ElapsedMilliseconds
        
        $results += [PSCustomObject]@{
            Iteration = $i
            Type = if ($i -eq 1) { "Cold" } else { "Warm" }
            StartupTime_ms = $elapsedMs
            Ready = $ready
        }
        
        Write-Host "  $ProcessName Run $i ($([string](if ($i -eq 1) { 'Cold' } else { 'Warm' }))): $elapsedMs ms"
        
        # Cleanup
        try {
            Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
            Get-Process -Name "msedgewebview2" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
        } catch {}
        Start-Sleep -Milliseconds 500
    }
    return $results
}

$gpuiStartup = Benchmark-Startup -ExePath $gpuiExe -ProcessName "MindPlace-GPUI"
$webview2Startup = Benchmark-Startup -ExePath $webview2Exe -ProcessName "MindPlace-WebView2"

# 3. Sustained Memory & CPU Profiling (30s Idle + Interaction)
Write-Host "`nProfiling GPUI Sustained Lifecycle..."
Get-Process -Name "MindPlace-GPUI", "MindPlace-WebView2", "msedgewebview2" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

$gpuiProc = Start-Process -FilePath $gpuiExe -PassThru
Start-Sleep -Seconds 2
$gpuiMemInitial = Measure-ProcessTreeMemory -RootPid $gpuiProc.Id
Write-Host "  GPUI Initial Launch Memory: WorkingSet = $($gpuiMemInitial.Total_WorkingSet_MB) MB, Private = $($gpuiMemInitial.Total_PrivateBytes_MB) MB (Processes: $($gpuiMemInitial.ProcessCount))"

# Sample CPU over 30s idle
$gpuiCpuSamples = @()
$prevTime = [DateTime]::UtcNow
$gpuiProc.Refresh()
$prevCpu = $gpuiProc.TotalProcessorTime.TotalMilliseconds
$sampleCount = 15

for ($s = 1; $s -le $sampleCount; $s++) {
    Start-Sleep -Seconds 2
    $currTime = [DateTime]::UtcNow
    $gpuiProc.Refresh()
    $currCpu = $gpuiProc.TotalProcessorTime.TotalMilliseconds
    $timeDelta = ($currTime - $prevTime).TotalMilliseconds
    $cpuDelta = $currCpu - $prevCpu
    $cpuPercent = ($cpuDelta / ($timeDelta * [Environment]::ProcessorCount)) * 100
    $gpuiCpuSamples += $cpuPercent
    $prevTime = $currTime
    $prevCpu = $currCpu
}

$gpuiMemIdle30s = Measure-ProcessTreeMemory -RootPid $gpuiProc.Id
Write-Host "  GPUI 30s Idle Memory: WorkingSet = $($gpuiMemIdle30s.Total_WorkingSet_MB) MB, Private = $($gpuiMemIdle30s.Total_PrivateBytes_MB) MB"

# Interaction Phase (simulate navigation/input)
Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.SendKeys]::SendWait("^{k}") # Command palette
Start-Sleep -Milliseconds 300
[System.Windows.Forms.SendKeys]::SendWait("graph{ENTER}")
Start-Sleep -Milliseconds 500
[System.Windows.Forms.SendKeys]::SendWait("{TAB}{TAB}{ENTER}")
Start-Sleep -Seconds 2

$gpuiMemInteractive = Measure-ProcessTreeMemory -RootPid $gpuiProc.Id
Write-Host "  GPUI Interactive Memory: WorkingSet = $($gpuiMemInteractive.Total_WorkingSet_MB) MB, Private = $($gpuiMemInteractive.Total_PrivateBytes_MB) MB"

# Graceful Exit test
$gpuiProc.CloseMainWindow() | Out-Null
Start-Sleep -Seconds 2
$gpuiExitClean = $gpuiProc.HasExited
if (!$gpuiExitClean) { Stop-Process -Id $gpuiProc.Id -Force -ErrorAction SilentlyContinue }

# Now Profile WebView2 Sustained Lifecycle
Write-Host "`nProfiling WebView2 Sustained Lifecycle..."
Get-Process -Name "MindPlace-WebView2", "msedgewebview2" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

$wvProc = Start-Process -FilePath $webview2Exe -PassThru
Start-Sleep -Seconds 2
$wvMemInitial = Measure-ProcessTreeMemory -RootPid $wvProc.Id
Write-Host "  WebView2 Initial Launch Memory: WorkingSet = $($wvMemInitial.Total_WorkingSet_MB) MB, Private = $($wvMemInitial.Total_PrivateBytes_MB) MB (Processes: $($wvMemInitial.ProcessCount))"
foreach ($d in $wvMemInitial.Details) {
    Write-Host "    - PID $($d.Id) ($($d.Name)): WS = $($d.WorkingSet_MB) MB, Private = $($d.PrivateBytes_MB) MB"
}

# Sample CPU over 30s idle across the entire tree
$wvCpuSamples = @()
$treeProcs = Get-ProcessTree -RootPid $wvProc.Id
$prevTreeCpu = ($treeProcs | ForEach-Object { $_.TotalProcessorTime.TotalMilliseconds } | Measure-Object -Sum).Sum
$prevTime = [DateTime]::UtcNow

for ($s = 1; $s -le $sampleCount; $s++) {
    Start-Sleep -Seconds 2
    $currTime = [DateTime]::UtcNow
    $treeProcs = Get-ProcessTree -RootPid $wvProc.Id
    $currTreeCpu = ($treeProcs | ForEach-Object { 
        try { $_.Refresh(); $_.TotalProcessorTime.TotalMilliseconds } catch { 0 }
    } | Measure-Object -Sum).Sum
    
    $timeDelta = ($currTime - $prevTime).TotalMilliseconds
    $cpuDelta = [Math]::Max(0, ($currTreeCpu - $prevTreeCpu))
    $cpuPercent = ($cpuDelta / ($timeDelta * [Environment]::ProcessorCount)) * 100
    $wvCpuSamples += $cpuPercent
    $prevTime = $currTime
    $prevTreeCpu = $currTreeCpu
}

$wvMemIdle30s = Measure-ProcessTreeMemory -RootPid $wvProc.Id
Write-Host "  WebView2 30s Idle Memory: WorkingSet = $($wvMemIdle30s.Total_WorkingSet_MB) MB, Private = $($wvMemIdle30s.Total_PrivateBytes_MB) MB (Processes: $($wvMemIdle30s.ProcessCount))"

# Interaction Phase (simulate navigation/input)
[System.Windows.Forms.SendKeys]::SendWait("^{k}")
Start-Sleep -Milliseconds 300
[System.Windows.Forms.SendKeys]::SendWait("graph{ENTER}")
Start-Sleep -Milliseconds 500
[System.Windows.Forms.SendKeys]::SendWait("{TAB}{TAB}{ENTER}")
Start-Sleep -Seconds 2

$wvMemInteractive = Measure-ProcessTreeMemory -RootPid $wvProc.Id
Write-Host "  WebView2 Interactive Memory: WorkingSet = $($wvMemInteractive.Total_WorkingSet_MB) MB, Private = $($wvMemInteractive.Total_PrivateBytes_MB) MB (Processes: $($wvMemInteractive.ProcessCount))"

# Graceful Exit test
$wvProc.CloseMainWindow() | Out-Null
Start-Sleep -Seconds 2
$wvExitClean = $wvProc.HasExited
# Check if child processes also exited
$orphans = Get-Process -Name "msedgewebview2" -ErrorAction SilentlyContinue
$wvOrphansClean = ($orphans -eq $null -or $orphans.Count -eq 0)

if (!$wvExitClean) { Stop-Process -Id $wvProc.Id -Force -ErrorAction SilentlyContinue }
if (!$wvOrphansClean) { $orphans | Stop-Process -Force -ErrorAction SilentlyContinue }

# Summary Calculations
function Get-Stats {
    param([double[]]$Values)
    $sorted = $Values | Sort-Object
    $count = $sorted.Count
    $sum = ($sorted | Measure-Object -Sum).Sum
    $mean = [Math]::Round($sum / $count, 2)
    $median = if ($count % 2 -eq 1) {
        $sorted[[Math]::Floor($count / 2)]
    } else {
        [Math]::Round(($sorted[($count/2) - 1] + $sorted[$count/2]) / 2, 2)
    }
    return [PSCustomObject]@{
        Min = $sorted[0]
        Max = $sorted[$count - 1]
        Mean = $mean
        Median = $median
    }
}

$gpuiStartupWarmTimes = @($gpuiStartup[1..9] | ForEach-Object { $_.StartupTime_ms })
$wvStartupWarmTimes = @($webview2Startup[1..9] | ForEach-Object { $_.StartupTime_ms })

$gpuiWarmStats = Get-Stats -Values $gpuiStartupWarmTimes
$wvWarmStats = Get-Stats -Values $wvStartupWarmTimes
$gpuiCpuStats = Get-Stats -Values $gpuiCpuSamples
$wvCpuStats = Get-Stats -Values $wvCpuSamples

$benchmarkResults = [PSCustomObject]@{
    System = [PSCustomObject]@{
        OS = (Get-CimInstance Win32_OperatingSystem).Caption
        OS_Version = (Get-CimInstance Win32_OperatingSystem).Version
        Processor = (Get-CimInstance Win32_Processor).Name
        LogicalCores = [Environment]::ProcessorCount
        TotalPhysicalRAM_GB = [Math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)
        Timestamp = (Get-Date).ToString("o")
    }
    Footprint = $footprint
    Startup = [PSCustomObject]@{
        GPUI = [PSCustomObject]@{
            Cold_ms = $gpuiStartup[0].StartupTime_ms
            Warm_Min_ms = $gpuiWarmStats.Min
            Warm_Max_ms = $gpuiWarmStats.Max
            Warm_Mean_ms = $gpuiWarmStats.Mean
            Warm_Median_ms = $gpuiWarmStats.Median
            AllRuns_ms = @($gpuiStartup | ForEach-Object { $_.StartupTime_ms })
        }
        WebView2 = [PSCustomObject]@{
            Cold_ms = $webview2Startup[0].StartupTime_ms
            Warm_Min_ms = $wvWarmStats.Min
            Warm_Max_ms = $wvWarmStats.Max
            Warm_Mean_ms = $wvWarmStats.Mean
            Warm_Median_ms = $wvWarmStats.Median
            AllRuns_ms = @($webview2Startup | ForEach-Object { $_.StartupTime_ms })
        }
    }
    Memory = [PSCustomObject]@{
        InitialLaunch = [PSCustomObject]@{
            GPUI_WorkingSet_MB = $gpuiMemInitial.Total_WorkingSet_MB
            GPUI_PrivateBytes_MB = $gpuiMemInitial.Total_PrivateBytes_MB
            GPUI_ProcessCount = $gpuiMemInitial.ProcessCount
            WebView2_WorkingSet_MB = $wvMemInitial.Total_WorkingSet_MB
            WebView2_PrivateBytes_MB = $wvMemInitial.Total_PrivateBytes_MB
            WebView2_ProcessCount = $wvMemInitial.ProcessCount
            WebView2_Breakdown = $wvMemInitial.Details
        }
        Idle30s = [PSCustomObject]@{
            GPUI_WorkingSet_MB = $gpuiMemIdle30s.Total_WorkingSet_MB
            GPUI_PrivateBytes_MB = $gpuiMemIdle30s.Total_PrivateBytes_MB
            WebView2_WorkingSet_MB = $wvMemIdle30s.Total_WorkingSet_MB
            WebView2_PrivateBytes_MB = $wvMemIdle30s.Total_PrivateBytes_MB
            WebView2_ProcessCount = $wvMemIdle30s.ProcessCount
        }
        Interactive = [PSCustomObject]@{
            GPUI_WorkingSet_MB = $gpuiMemInteractive.Total_WorkingSet_MB
            GPUI_PrivateBytes_MB = $gpuiMemInteractive.Total_PrivateBytes_MB
            WebView2_WorkingSet_MB = $wvMemInteractive.Total_WorkingSet_MB
            WebView2_PrivateBytes_MB = $wvMemInteractive.Total_PrivateBytes_MB
            WebView2_ProcessCount = $wvMemInteractive.ProcessCount
        }
    }
    CPU = [PSCustomObject]@{
        Idle_30s = [PSCustomObject]@{
            GPUI_Mean_Percent = $gpuiCpuStats.Mean
            GPUI_Max_Percent = $gpuiCpuStats.Max
            WebView2_Mean_Percent = $wvCpuStats.Mean
            WebView2_Max_Percent = $wvCpuStats.Max
        }
    }
    Lifecycle = [PSCustomObject]@{
        GPUI_CleanExit = $gpuiExitClean
        WebView2_HostCleanExit = $wvExitClean
        WebView2_OrphansCleanExit = $wvOrphansClean
    }
}

$jsonPath = Join-Path $outputDir "results.json"
$benchmarkResults | ConvertTo-Json -Depth 6 | Set-Content -Path $jsonPath -Encoding utf8
Write-Host "`nBenchmark results written to $jsonPath"
