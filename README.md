# Mind Place UI Regression & Architecture Benchmark
### Controlled Technology Benchmark: Native Rust GPUI vs Microsoft Edge WebView2

This repository contains the complete implementation, visual parity reconstruction, and empirical performance benchmarks for **Mind Place**—a monochrome knowledge OS interface evaluated across two distinct desktop rendering architectures:
1. **Native GPUI (`MindPlace-GPUI.exe`):** 100% native Rust desktop application built on GPUI (Direct3D 11 backend), featuring custom GPU quad streaming and immediate-mode layout.
2. **Microsoft Edge WebView2 (`MindPlace-WebView2.exe`):** A lightweight native Windows host (Rust Tao/Wry) embedding the Evergreen Microsoft Edge Chromium WebView2 engine with the original HTML/CSS/Canvas 2D client.

---

## Benchmark Highlights

### 1. Phase 1: Idle Footprint & Memory Baseline
- **GPUI Idle Working Set:** **~46 MB – 76 MB** (1 process)
- **WebView2 Idle Working Set:** **~392 MB** across 6 Chromium processes (Browser, GPU, Renderers, Crashpad)
- **Memory Advantage:** GPUI consumes **5.1x to 8.5x less memory**.

### 2. Phase 2: Graph Scalability & Stress Benchmark

Tested on a bare-metal 13th Gen Intel Core i7-13700HX (24 threads, 144 Hz display):

| Scenario / Workload | Native GPUI | Edge WebView2 | Verdict |
| :--- | :--- | :--- | :--- |
| **100 Nodes Stress** | **144.23 FPS** (Monitor Max) | 143.22 FPS | **Parity** (Both hit 144 Hz refresh cap) |
| **1,000 Nodes Stress** | **101.77 FPS** (P50: 8.72 ms) | 38.25 FPS (P50: 27.60 ms) | **GPUI is 2.66x faster (Smooth vs Sub-40 FPS jank)** |
| **10,000 Nodes Stress** | **5.14 FPS** (P50: 195.6 ms) | 3.00 FPS (P50: 319.3 ms) | **GPUI is 1.71x faster (+71% throughput)** |
| **25,000 Nodes Stress** | **2.39 FPS** | 1.33 FPS | **GPUI is 1.80x faster** |
| **1,000 Nodes Pan (10s)** | **107.97 FPS** (2.85% CPU) | 35.88 FPS (5.03% CPU) | **GPUI is 3.0x smoother with 43% lower CPU** |
| **1,000 Nodes Zoom (10s)** | **82.40 FPS** (2.73% CPU) | 29.80 FPS (5.83% CPU) | **GPUI is 2.76x faster** |
| **Memory at 1,000 Nodes** | **103.40 MB** Peak RAM | 524.67 MB Peak RAM | **GPUI uses 5.1x less RAM** |
| **Process Tree** | **1 Process** | 6 Processes | **Single-process zero-IPC deterministic lifecycle** |

Full empirical test results and analysis are documented in [`comparison/benchmarks/graph-stress/results.md`](comparison/benchmarks/graph-stress/results.md).

---

## Repository Structure

```
.
├── src/                                  # Native GPUI implementation (Rust)
│   ├── main.rs                           # Entrypoint, window lifecycle & CLI parser
│   ├── app_state.rs                      # Unified in-memory state & mock datasets
│   ├── theme.rs                          # Exact monochrome design tokens & palette
│   ├── screens/                          # 8 core screens (Overview, Archive, Vault, Hive, etc.)
│   └── bin/generate_datasets.rs          # Deterministic graph generator
├── mind_place_webview2/                  # WebView2 implementation
│   ├── assets/index.html                 # Original HTML/CSS/Canvas prototype + telemetry
│   └── src/main.rs                       # Rust Tao/Wry native host & IPC handler
├── comparison/                           # Distributables, screenshots & benchmark suites
│   ├── gpui/MindPlace-GPUI.exe           # Standalone GPUI executable (Release)
│   ├── webview2/MindPlace-WebView2.exe   # Standalone WebView2 executable (Release)
│   ├── screenshots/                      # Phase 1 & 2 visual parity screenshots
│   └── benchmarks/
│       ├── results.md                    # Phase 1 benchmark report
│       └── graph-stress/                 # Phase 2 stress benchmark suite
│           ├── datasets/                 # Shared JSON datasets (100, 1k, 10k, 25k nodes)
│           ├── screenshots/              # Native visual parity captures
│           ├── scripts/                  # Automated PowerShell test runners
│           ├── results.json              # Full raw metrics telemetry
│           └── results.md                # Comprehensive Phase 2 benchmark report
├── Cargo.toml                            # GPUI workspace manifest
└── README.md                             # This overview
```

---

## Running the Applications

Pre-compiled release binaries are included in the repository:

### 1. Launch Native GPUI
```powershell
.\comparison\gpui\MindPlace-GPUI.exe
```

### 2. Launch WebView2
```powershell
.\comparison\webview2\MindPlace-WebView2.exe
```

### 3. Interactive Dataset Switching
In both applications, open the **Hive Mind** screen from the sidebar. Use the dataset toggle buttons in the top-right toolbar (`33`, `100`, `1k`, `10k`) to switch graph density in real time.

---

## Reproducing the Benchmark Suite

Run the automated benchmark runner:

```powershell
powershell -ExecutionPolicy Bypass -File "comparison\benchmarks\graph-stress\scripts\run_graph_stress_benchmarks.ps1"
```

Or run a fast 2-second smoke verification:
```powershell
powershell -ExecutionPolicy Bypass -File "comparison\benchmarks\graph-stress\scripts\run_graph_stress_benchmarks.ps1" -QuickTest
```

---

## License & Attribution

- Prototype design & specifications derived from the Mind Place monochrome knowledge OS interface.
- Benchmark telemetry, GPUI native engine port, and comparison runner developed for technology evaluation and UI regression testing.
