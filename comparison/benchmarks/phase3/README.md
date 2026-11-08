# Mind Place — Phase 3 Graph Scalability & Architecture Benchmark

## Overview

Phase 3 is an audited, statistically rigorous desktop benchmark evaluating **GPUI (Direct3D 11 via Lyon and custom shaders)** against **Microsoft Edge WebView2 (Chromium Canvas 2D & hardware-accelerated WebGL2)** on identical interactive knowledge graph workloads in Mind Place.

This benchmark builds directly on the findings of the independent Phase 2 source-code audit, eliminating previous measurement discrepancies, fixing geometry defects, implementing hardware-accelerated WebGL2 rendering in WebView2, and enforcing identical Level of Detail (LOD) and label visibility contracts.

---

## Key Questions Addressed

1. **Geometry Fidelity**: Does GPUI render true quadratic Bézier curves at scale rather than simplified axis-aligned bounding rectangles?
2. **Visual Parity**: Do both implementations render identical node counts, edge counts, and label subsets in steady state?
3. **WebGL2 vs Canvas 2D vs GPUI**: How does a modern WebGL2 instanced ribbon pipeline in WebView2 compare against GPUI's Direct3D 11 tessellation and Canvas 2D path rasterization?
4. **Memory Transparency**: How does dynamic process-tree Private Bytes and Working Set memory scale between GPUI's single native process and WebView2's multi-process Chromium architecture?
5. **Frame Cadence vs Presentation**: Distinguishing internal engine loop cadence from asynchronous display presentation pipeline timing.

---

## Directory Structure

```text
comparison/benchmarks/phase3/
├── audit-fixes.md                 # Detailed resolution of all Phase 2 audit findings
├── rendering-contract.md          # Visual specification, geometry formulas, and LOD thresholds
├── correctness-results.json       # Visual parity verification and node/edge/label counts
├── benchmark-results.json         # Raw benchmark telemetry (5 repetitions per configuration)
├── benchmark-results.md           # Comprehensive benchmark report, charts, and analysis
├── README.md                      # This directory guide
├── datasets/                      # Deterministic datasets with SHA-256 manifest
│   ├── manifest.json
│   ├── graph_100.json
│   ├── graph_1000.json
│   ├── graph_10000.json
│   └── graph_25000.json
├── screenshots/                   # Resting-frame camera-settled visual parity captures
│   ├── gpui_100.png
│   ├── gpui_1000.png
│   ├── gpui_10000.png
│   ├── webview2_canvas_100.png
│   ├── webview2_canvas_1000.png
│   ├── webview2_canvas_10000.png
│   ├── webview2_webgl2_100.png
│   ├── webview2_webgl2_1000.png
│   └── webview2_webgl2_10000.png
└── scripts/                       # Automated execution scripts
    ├── capture_phase3_screenshots.ps1
    └── run_phase3_benchmarks.ps1
```

---

## Running the Benchmarks

### 1. Visual Parity Screenshot Capture
Captures high-resolution resting-frame screenshots at $(600, 400, 1.0)$ with camera settled:
```powershell
powershell -ExecutionPolicy Bypass -File comparison\benchmarks\phase3\scripts\capture_phase3_screenshots.ps1
```

### 2. Full Benchmark Suite
Runs 5 repetitions across all 3 engines (`gpui`, `canvas`, `webgl2`), 2 modes (`full`, `lod`), and 4 scales (100, 1k, 10k, 25k nodes):
```powershell
powershell -ExecutionPolicy Bypass -File comparison\benchmarks\phase3\scripts\run_phase3_benchmarks.ps1 -Repetitions 5 -DurationSec 3.0
```

### 3. Individual Executable Execution
**GPUI:**
```powershell
.\comparison\gpui\MindPlace-GPUI.exe --benchmark comparison\benchmarks\phase3\datasets\graph_1000.json --mode full --scenario stress --duration 5
```

**WebView2 (Canvas 2D):**
```powershell
.\comparison\webview2\MindPlace-WebView2.exe --benchmark comparison\benchmarks\phase3\datasets\graph_1000.json --renderer canvas --mode full --scenario stress --duration 5
```

**WebView2 (WebGL2):**
```powershell
.\comparison\webview2\MindPlace-WebView2.exe --benchmark comparison\benchmarks\phase3\datasets\graph_1000.json --renderer webgl2 --mode full --scenario stress --duration 5
```
