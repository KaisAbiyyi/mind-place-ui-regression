# comparison/benchmarks/phase3/scripts/run_phase3_benchmarks.ps1
# Phase 3 Automated Benchmark Runner
# Measures GPUI, WebView2 (Canvas 2D), and WebView2 (WebGL2) under identical workloads

param(
    [int]$Repetitions = 5,
    [double]$DurationSec = 5.0,
    [string[]]$Datasets = @("graph_100.json", "graph_1000.json", "graph_10000.json", "graph_25000.json"),
    [string[]]$Renderers = @("gpui", "canvas", "webgl2"),
    [string[]]$Modes = @("full", "lod"),
    [string]$OutputFile = "comparison\benchmarks\phase3\benchmark-results.json"
)

function Get-ProcessTreeStats {
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

    $totalWorkingSet = 0L
    $totalPrivateBytes = 0L
    $procCount = 0

    foreach ($pidVal in $allPids) {
        $proc = Get-Process -Id $pidVal -ErrorAction SilentlyContinue
        if ($proc) {
            $totalWorkingSet += $proc.WorkingSet64
            $totalPrivateBytes += $proc.PrivateMemorySize64
            $procCount++
        }
    }

    return @{
        ProcessCount = $procCount
        WorkingSetMB = [math]::Round($totalWorkingSet / 1MB, 2)
        PrivateBytesMB = [math]::Round($totalPrivateBytes / 1MB, 2)
        Pids = $allPids
    }
}

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
    foreach ($pidVal in $allPids) {
        try {
            Stop-Process -Id $pidVal -Force -ErrorAction SilentlyContinue
        } catch {}
    }
}

if ($Datasets.Count -eq 1 -and $Datasets[0].Contains(',')) { $Datasets = $Datasets[0].Split(',') | ForEach-Object { $_.Trim() } }
if ($Renderers.Count -eq 1 -and $Renderers[0].Contains(',')) { $Renderers = $Renderers[0].Split(',') | ForEach-Object { $_.Trim() } }
if ($Modes.Count -eq 1 -and $Modes[0].Contains(',')) { $Modes = $Modes[0].Split(',') | ForEach-Object { $_.Trim() } }

$root = (Get-Item -Path ".").FullName
$gpuiExe = Join-Path $root "comparison\gpui\MindPlace-GPUI.exe"
$wvExe = Join-Path $root "comparison\webview2\MindPlace-WebView2.exe"
$datasetsDir = Join-Path $root "comparison\benchmarks\phase3\datasets"

$allResults = @()

Write-Host "==========================================================" -ForegroundColor Green
Write-Host " Starting Phase 3 Benchmark Suite" -ForegroundColor Green
Write-Host " Repetitions: $Repetitions | Duration: ${DurationSec}s" -ForegroundColor Green
Write-Host " Datasets: $($Datasets -join ', ')" -ForegroundColor Green
Write-Host " Renderers: $($Renderers -join ', ')" -ForegroundColor Green
Write-Host " Modes: $($Modes -join ', ')" -ForegroundColor Green
Write-Host "==========================================================" -ForegroundColor Green

foreach ($dsName in $Datasets) {
    $dsPath = Join-Path $datasetsDir $dsName
    if (-not (Test-Path $dsPath)) {
        Write-Warning "Dataset not found: $dsPath, skipping..."
        continue
    }

    $dsInfo = Get-Content $dsPath -Raw | ConvertFrom-Json
    $nodeCount = if ($dsInfo.node_count) { $dsInfo.node_count } else { $dsInfo.nodes.Count }
    $edgeCount = if ($dsInfo.edge_count) { $dsInfo.edge_count } else { $dsInfo.edges.Count }

    foreach ($mode in $Modes) {
        foreach ($rend in $Renderers) {
            Write-Host "`n>>> Running: Dataset $dsName ($nodeCount nodes) | Mode: $mode | Renderer: $rend" -ForegroundColor Cyan

            $repResults = @()

            for ($rep = 1; $rep -le $Repetitions; $rep++) {
                Write-Host "  Iteration $rep/$Repetitions..." -NoNewline

                $tempOut = [System.IO.Path]::GetTempFileName()
                $argList = @()
                $exeToRun = ""

                if ($rend -eq "gpui") {
                    $exeToRun = $gpuiExe
                    $argList = @(
                        "--benchmark", "`"$dsPath`"",
                        "--scenario", "stress",
                        "--duration", [string]$DurationSec,
                        "--mode", $mode,
                        "--output", "`"$tempOut`""
                    )
                } else {
                    $exeToRun = $wvExe
                    $argList = @(
                        "--benchmark", "`"$dsPath`"",
                        "--scenario", "stress",
                        "--duration", [string]$DurationSec,
                        "--renderer", $rend,
                        "--mode", $mode,
                        "--output", "`"$tempOut`""
                    )
                }

                $argList = @(
                    "--benchmark", "`"$dsPath`"",
                    "--scenario", "stress",
                    "--duration", [string]$DurationSec,
                    "--mode", $mode,
                    "--output", "`"$tempOut`""
                )
                if ($rend -ne "gpui") {
                    $argList += @("--renderer", $rend)
                }

                $p = Start-Process -FilePath $exeToRun -ArgumentList $argList -PassThru

                # Wait for initial spinup (1.5s), then sample process tree memory at steady state
                Start-Sleep -Milliseconds 1800
                $memStats = Get-ProcessTreeStats -ParentPid $p.Id

                # Wait for process exit (max duration + 10s)
                $timeoutSec = [int]($DurationSec + 10)
                $p.WaitForExit($timeoutSec * 1000) | Out-Null

                # Clean up process tree safely
                Stop-ProcessTreeSafe -ParentPid $p.Id

                # Read telemetry JSON from output file
                $jsonStr = ""
                if (Test-Path $tempOut) {
                    $jsonStr = Get-Content $tempOut -Raw
                    Remove-Item $tempOut -Force -ErrorAction SilentlyContinue
                }

                    if ($jsonStr) {
                        try {
                            $benchJson = $jsonStr | ConvertFrom-Json
                            
                            $sample = @{
                                repetition = $rep
                                renderer = $rend
                                mode = $mode
                                dataset = $dsName
                                node_count = $nodeCount
                                edge_count = $edgeCount
                                total_frames = $benchJson.total_frames
                                mean_frame_time_ms = $benchJson.mean_frame_time_ms
                                avg_frame_cadence_fps = $benchJson.avg_frame_cadence_fps
                                p50_frame_time_ms = $benchJson.p50_frame_time_ms
                                p95_frame_time_ms = $benchJson.p95_frame_time_ms
                                p99_frame_time_ms = $benchJson.p99_frame_time_ms
                                min_frame_time_ms = $benchJson.min_frame_time_ms
                                max_frame_time_ms = $benchJson.max_frame_time_ms
                                mean_cpu_draw_ms = $benchJson.mean_cpu_draw_ms
                                mean_cpu_update_ms = $benchJson.mean_cpu_update_ms
                                mean_nodes_rendered = $benchJson.mean_nodes_rendered
                                mean_edges_rendered = $benchJson.mean_edges_rendered
                                mean_labels_rendered = $benchJson.mean_labels_rendered
                                mean_primitives_culled = $benchJson.mean_primitives_culled
                                dataset_load_time_ms = $benchJson.dataset_load_time_ms
                                working_set_mb = $memStats.WorkingSetMB
                                private_bytes_mb = $memStats.PrivateBytesMB
                                process_count = $memStats.ProcessCount
                            }

                            $repResults += $sample
                            Write-Host " FPS: $($sample.avg_frame_cadence_fps) | CPU Draw: $($sample.mean_cpu_draw_ms)ms | WS: $($sample.working_set_mb)MB | Priv: $($sample.private_bytes_mb)MB" -ForegroundColor Gray
                        } catch {
                            Write-Host " [JSON Parse Error: $_]" -ForegroundColor Yellow
                        }
                    } else {
                        Write-Host " [No Telemetry Output]" -ForegroundColor Red
                    }

                Start-Sleep -Milliseconds 500
            }

            if ($repResults.Count -gt 0) {
                # Aggregate statistics
                $fpsArr = $repResults | ForEach-Object { $_.avg_frame_cadence_fps }
                $drawArr = $repResults | ForEach-Object { $_.mean_cpu_draw_ms }
                $wsArr = $repResults | ForEach-Object { $_.working_set_mb }
                $privArr = $repResults | ForEach-Object { $_.private_bytes_mb }

                $avgFps = [math]::Round(($fpsArr | Measure-Object -Average).Average, 2)
                $avgDraw = [math]::Round(($drawArr | Measure-Object -Average).Average, 3)
                $avgWs = [math]::Round(($wsArr | Measure-Object -Average).Average, 2)
                $avgPriv = [math]::Round(($privArr | Measure-Object -Average).Average, 2)

                $summary = @{
                    renderer = $rend
                    mode = $mode
                    dataset = $dsName
                    node_count = $nodeCount
                    edge_count = $edgeCount
                    valid_runs = $repResults.Count
                    avg_frame_cadence_fps = $avgFps
                    mean_cpu_draw_ms = $avgDraw
                    mean_cpu_update_ms = [math]::Round((($repResults | ForEach-Object { $_.mean_cpu_update_ms } | Measure-Object -Average).Average), 3)
                    p50_frame_time_ms = [math]::Round((($repResults | ForEach-Object { $_.p50_frame_time_ms } | Measure-Object -Average).Average), 2)
                    p95_frame_time_ms = [math]::Round((($repResults | ForEach-Object { $_.p95_frame_time_ms } | Measure-Object -Average).Average), 2)
                    p99_frame_time_ms = [math]::Round((($repResults | ForEach-Object { $_.p99_frame_time_ms } | Measure-Object -Average).Average), 2)
                    mean_nodes_rendered = [math]::Round((($repResults | ForEach-Object { $_.mean_nodes_rendered } | Measure-Object -Average).Average), 1)
                    mean_edges_rendered = [math]::Round((($repResults | ForEach-Object { $_.mean_edges_rendered } | Measure-Object -Average).Average), 1)
                    mean_labels_rendered = [math]::Round((($repResults | ForEach-Object { $_.mean_labels_rendered } | Measure-Object -Average).Average), 1)
                    mean_primitives_culled = [math]::Round((($repResults | ForEach-Object { $_.mean_primitives_culled } | Measure-Object -Average).Average), 1)
                    dataset_load_time_ms = [math]::Round((($repResults | ForEach-Object { $_.dataset_load_time_ms } | Measure-Object -Average).Average), 2)
                    working_set_mb = $avgWs
                    private_bytes_mb = $avgPriv
                    process_count = ($repResults | Select-Object -First 1).process_count
                    runs = $repResults
                }

                $allResults += $summary
                Write-Host "  ===> SUMMARY: Cadence FPS: $avgFps | Draw: ${avgDraw}ms | WorkingSet: ${avgWs}MB | Private: ${avgPriv}MB" -ForegroundColor Green
            }
        }
    }
}

$fullJson = @{
    generated_at = (Get-Date).ToString("o")
    environment = @{
        os = "Windows 11 x64"
        powershell_version = $PSVersionTable.PSVersion.ToString()
        repetitions_per_scenario = $Repetitions
        duration_per_run_sec = $DurationSec
        warmup_discard_ms = 500
    }
    results = $allResults
} | ConvertTo-Json -Depth 10

$outFullPath = Join-Path $root $OutputFile
$outDir = Split-Path $outFullPath -Parent
if (-not (Test-Path $outDir)) { New-Item -ItemType Directory -Path $outDir -Force | Out-Null }
[System.IO.File]::WriteAllText($outFullPath, $fullJson)
Write-Host "`nAll benchmark runs completed! Results saved to: $outFullPath" -ForegroundColor Green
