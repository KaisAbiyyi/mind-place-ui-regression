Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Runtime.InteropServices;
public class WinUtil3 {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr hwnd, IntPtr hdcBlt, uint nFlags);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
}
'@

function Capture-Shot($exe, $ds, $outFile, $delaySec) {
    Write-Host "Launching $exe with $ds..."
    $arg = "--benchmark `"$ds`" --scenario stress --duration 10"
    $p = Start-Process -FilePath $exe -ArgumentList $arg -PassThru
    Start-Sleep -Seconds $delaySec
    
    $proc = Get-Process -Id $p.Id
    $hWnd = $proc.MainWindowHandle
    if ($hWnd -eq [IntPtr]::Zero) {
        $children = Get-CimInstance Win32_Process | Where-Object { $_.ParentProcessId -eq $p.Id }
        foreach ($c in $children) {
            $cp = Get-Process -Id $c.ProcessId -ErrorAction SilentlyContinue
            if ($cp -and $cp.MainWindowHandle -ne [IntPtr]::Zero) {
                $hWnd = $cp.MainWindowHandle
                break
            }
        }
    }
    
    if ($hWnd -eq [IntPtr]::Zero) {
        Write-Warning "Could not find hWnd for process $p.Id"
        $p.WaitForExit()
        return
    }

    [WinUtil3]::ShowWindow($hWnd, 9) | Out-Null
    [WinUtil3]::SetForegroundWindow($hWnd) | Out-Null
    Start-Sleep -Milliseconds 600

    $rect = New-Object WinUtil3+RECT
    [WinUtil3]::GetWindowRect($hWnd, [ref]$rect) | Out-Null
    $w = $rect.Right - $rect.Left
    $h = $rect.Bottom - $rect.Top

    if ($w -gt 200 -and $h -gt 200) {
        $bmp = New-Object System.Drawing.Bitmap($w, $h)
        $g = [System.Drawing.Graphics]::FromImage($bmp)
        $hdc = $g.GetHdc()
        $ok = [WinUtil3]::PrintWindow($hWnd, $hdc, 2)
        $g.ReleaseHdc($hdc)
        if (!$ok) {
            $g.CopyFromScreen($rect.Left, $rect.Top, 0, 0, (New-Object System.Drawing.Size($w, $h)))
        }
        $bmp.Save($outFile, [System.Drawing.Imaging.ImageFormat]::Png)
        $g.Dispose()
        $bmp.Dispose()
        Write-Host "Successfully saved $outFile ($w x $h)" -ForegroundColor Green
    }

    $p.WaitForExit()
}

$root = (Get-Item -Path ".").FullName
$wvExe = Join-Path $root "comparison\webview2\MindPlace-WebView2.exe"
$ds1000 = Join-Path $root "comparison\benchmarks\graph-stress\datasets\graph_1000.json"
$ds10000 = Join-Path $root "comparison\benchmarks\graph-stress\datasets\graph_10000.json"
$out1000 = Join-Path $root "comparison\benchmarks\graph-stress\screenshots\webview2_1000.png"
$out10000 = Join-Path $root "comparison\benchmarks\graph-stress\screenshots\webview2_10000.png"

Capture-Shot $wvExe $ds1000 $out1000 3
Capture-Shot $wvExe $ds10000 $out10000 4
