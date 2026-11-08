# Mind Place — Phase 2: Graph Scalability & Stress Benchmark

This directory contains the complete reproducibility suite for the **Phase 2 Graph Scalability Benchmark**, comparing the native **Rust GPUI** implementation against the **Microsoft Edge WebView2** implementation across datasets of 100, 1,000, 10,000, and 25,000 nodes.

---

## Directory Structure

```
comparison/benchmarks/graph-stress/
├── datasets/                          # Shared, deterministic graph datasets
│   ├── graph_100.json                 # 100 nodes, 307 edges
│   ├── graph_1000.json                # 1,000 nodes, 1,796 edges
│   ├── graph_10000.json               # 10,000 nodes, 14,199 edges
│   └── graph_25000.json               # 25,000 nodes, 35,008 edges
├── screenshots/                       # Visual parity verification captures
│   ├── gpui_100.png                   # GPUI 100 nodes capture
│   ├── webview2_100.png               # WebView2 100 nodes capture
│   ├── gpui_1000.png                  # GPUI 1,000 nodes capture
│   ├── webview2_1000.png              # WebView2 1,000 nodes capture
│   ├── gpui_10000.png                 # GPUI 10,000 nodes capture
│   └── webview2_10000.png             # WebView2 10,000 nodes capture
├── scripts/
│   ├── run_graph_stress_benchmarks.ps1 # Full automated runner & telemetry sampler
│   └── capture_screenshots.ps1         # Standalone visual parity capture tool
├── results.json                       # Complete empirical JSON metrics
├── results.md                         # Detailed benchmark analysis & comparison report
└── README.md                          # This file
```

---

## Quick Start: Running the Benchmarks

### 1. Run the Full Automated Benchmark Suite
To execute all benchmarks (10s continuous runs for 100, 1,000, 10,000, and 25,000 nodes plus individual interaction scenarios) and record full process telemetry into `results.json`:

```powershell
powershell -ExecutionPolicy Bypass -File "comparison\benchmarks\graph-stress\scripts\run_graph_stress_benchmarks.ps1"
```

### 2. Fast Verification Run (2-Second Smoke Test)
To verify that all binaries, scenarios, and process monitors work without waiting for full durations:

```powershell
powershell -ExecutionPolicy Bypass -File "comparison\benchmarks\graph-stress\scripts\run_graph_stress_benchmarks.ps1" -QuickTest
```

---

## Standalone CLI Usage

Both executables accept standard CLI flags to run specific benchmark scenarios headlessly or with telemetry output:

### Native GPUI (`MindPlace-GPUI.exe`)
```powershell
# Run a 10-second Pan benchmark with 1,000 nodes
.\comparison\gpui\MindPlace-GPUI.exe --benchmark "comparison\benchmarks\graph-stress\datasets\graph_1000.json" --scenario pan --duration 10 --output "gpui_pan_results.json"
```

### Microsoft Edge WebView2 (`MindPlace-WebView2.exe`)
```powershell
# Run the same 10-second Pan benchmark with WebView2
.\comparison\webview2\MindPlace-WebView2.exe --benchmark "comparison\benchmarks\graph-stress\datasets\graph_1000.json" --scenario pan --duration 10 --output "wv_pan_results.json"
```

### Available CLI Flags:
- `--benchmark <file>`: Path to graph JSON dataset.
- `--scenario <name>`: One of `stress`, `pan`, `zoom`, `hover`, `select`, `navigate`.
- `--duration <seconds>`: Duration of the benchmark run (defaults to 10s).
- `--output <file>`: Path where telemetry JSON metrics are saved on completion.

---

## Interactive UI Dataset Switcher

Both desktop applications include interactive buttons in the **Hive Mind** screen header toolbar to switch between datasets in real time:

- **`33`**: Original Mind Place mock dataset (33 nodes).
- **`100`**: Light synthetic benchmark dataset (100 nodes, 307 edges).
- **`1k`**: Moderate synthetic benchmark dataset (1,000 nodes, 1,796 edges).
- **`10k`**: Heavy synthetic benchmark dataset (10,000 nodes, 14,199 edges).

Simply click any dataset button in the top-right toolbar of the Hive Mind graph to immediately switch the visualizer and observe performance interactively.
