# comparison/benchmarks/phase3/scripts/capture_phase3_screenshots.ps1
# Deterministic post-camera-motion stable screenshot capture for Phase 3 visual parity audit

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Runtime.InteropServices;
public class WinUtilP3 {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr hwnd, IntPtr hdcBlt, uint nFlags);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
}
'@

function Stop-ProcessTreeSafe {
    param([int]$ParentPid)
    $queue = [System.Collections.Generic.Queue[int]]::new()
    $queue.Enqueue($ParentPid)
    $allPids = [System.Collections.Generic.List[int]]::new()
    $allPids.Add($ParentPid)
    
    while ($queue.Count -gt 0) {
        $curr = $queue.Dequeue()
        $children = Get-CimInstance Win32_Process -Filter "ParentProcessId = $curr" -ErrorAction SilentlyContinue
        if ($children) {
            foreach ($ch in $children) {
                if (-not $allPids.Contains($ch.ProcessId)) {
                    $allPids.Add($ch.ProcessId)
                    $queue.Enqueue($ch.ProcessId)
                }
            }
        }
    }
    
    $allPids.Reverse()
    foreach ($p in $allPids) {
        try {
            Stop-Process -Id $p -Force -ErrorAction SilentlyContinue
        } catch {}
    }
}

function Capture-StableShot {
    param(
        [string]$ExePath,
        [string]$DatasetPath,
        [string]$ExtraArgs,
        [string]$OutFile,
        [int]$SettleSec = 2
    )

    Write-Host "Capturing stable frame: $ExePath with $DatasetPath ($ExtraArgs)..." -ForegroundColor Cyan
    $argList = "--benchmark `"$DatasetPath`" --scenario idle --duration 15 $ExtraArgs"
    $p = Start-Process -FilePath $ExePath -ArgumentList $argList -PassThru

    Start-Sleep -Seconds $SettleSec

    $hWnd = [IntPtr]::Zero
    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    while ($hWnd -eq [IntPtr]::Zero -and $stopwatch.ElapsedMilliseconds -lt 5000) {
        $proc = Get-Process -Id $p.Id -ErrorAction SilentlyContinue
        if ($proc -and $proc.MainWindowHandle -ne [IntPtr]::Zero) {
            $hWnd = $proc.MainWindowHandle
            break
        }
        $children = Get-CimInstance Win32_Process -Filter "ParentProcessId = $($p.Id)" -ErrorAction SilentlyContinue
        foreach ($c in $children) {
            $cp = Get-Process -Id $c.ProcessId -ErrorAction SilentlyContinue
            if ($cp -and $cp.MainWindowHandle -ne [IntPtr]::Zero) {
                $hWnd = $cp.MainWindowHandle
                break
            }
        }
        Start-Sleep -Milliseconds 150
    }

    if ($hWnd -ne [IntPtr]::Zero) {
        [WinUtilP3]::ShowWindow($hWnd, 9) | Out-Null
        [WinUtilP3]::SetForegroundWindow($hWnd) | Out-Null
        Start-Sleep -Milliseconds 500

        $rect = New-Object WinUtilP3+RECT
        [WinUtilP3]::GetWindowRect($hWnd, [ref]$rect) | Out-Null
        $w = $rect.Right - $rect.Left
        $h = $rect.Bottom - $rect.Top

        if ($w -gt 200 -and $h -gt 200) {
            $bmp = New-Object System.Drawing.Bitmap($w, $h)
            $g = [System.Drawing.Graphics]::FromImage($bmp)
            $hdc = $g.GetHdc()
            $ok = [WinUtilP3]::PrintWindow($hWnd, $hdc, 2)
            $g.ReleaseHdc($hdc)
            if (!$ok) {
                $g.CopyFromScreen($rect.Left, $rect.Top, 0, 0, (New-Object System.Drawing.Size($w, $h)))
            }
            $bmp.Save($OutFile, [System.Drawing.Imaging.ImageFormat]::Png)
            $g.Dispose()
            $bmp.Dispose()
            Write-Host "  -> Saved: $OutFile ($w x $h)" -ForegroundColor Green
        }
    } else {
        Write-Warning "Failed to acquire hWnd for $ExePath"
    }

    Stop-ProcessTreeSafe $p.Id
    Start-Sleep -Milliseconds 400
}

$root = (Get-Item -Path ".").FullName
$gpuiExe = Join-Path $root "comparison\gpui\MindPlace-GPUI.exe"
$wvExe = Join-Path $root "comparison\webview2\MindPlace-WebView2.exe"

$ds100 = Join-Path $root "comparison\benchmarks\phase3\datasets\graph_100.json"
$ds1000 = Join-Path $root "comparison\benchmarks\phase3\datasets\graph_1000.json"
$ds10000 = Join-Path $root "comparison\benchmarks\phase3\datasets\graph_10000.json"

$outDir = Join-Path $root "comparison\benchmarks\phase3\screenshots"
if (-not (Test-Path $outDir)) { New-Item -ItemType Directory -Path $outDir -Force | Out-Null }

# 1. GPUI (Full Fidelity)
Capture-StableShot $gpuiExe $ds100 "--mode full" (Join-Path $outDir "gpui_100.png") 2
Capture-StableShot $gpuiExe $ds1000 "--mode full" (Join-Path $outDir "gpui_1000.png") 2
Capture-StableShot $gpuiExe $ds10000 "--mode full" (Join-Path $outDir "gpui_10000.png") 3

# 2. WebView2 Canvas 2D (Full Fidelity)
Capture-StableShot $wvExe $ds100 "--renderer canvas --mode full" (Join-Path $outDir "webview2_canvas_100.png") 2
Capture-StableShot $wvExe $ds1000 "--renderer canvas --mode full" (Join-Path $outDir "webview2_canvas_1000.png") 2
Capture-StableShot $wvExe $ds10000 "--renderer canvas --mode full" (Join-Path $outDir "webview2_canvas_10000.png") 3

# 3. WebView2 WebGL2 (Full Fidelity)
Capture-StableShot $wvExe $ds100 "--renderer webgl2 --mode full" (Join-Path $outDir "webview2_webgl2_100.png") 2
Capture-StableShot $wvExe $ds1000 "--renderer webgl2 --mode full" (Join-Path $outDir "webview2_webgl2_1000.png") 2
Capture-StableShot $wvExe $ds10000 "--renderer webgl2 --mode full" (Join-Path $outDir "webview2_webgl2_10000.png") 3

Write-Host "All Phase 3 stable screenshots captured successfully." -ForegroundColor Green
