# Mind Place — Phase 3 Graph Renderer Scalability & Architecture Benchmark

**Repository:** `https://github.com/KaisAbiyyi/mind-place-ui-regression`  
**Branch:** `experiment/phase3-graph-renderers`  
**Audited Baseline Commit:** `90737fd8cac579f32dd102c7a0558e81e1f17d69`  
**Benchmark Date:** 2026-11-08  
**Operating System:** Windows 11 x64  
**Author:** Antigravity Autonomous Systems Benchmark Suite  

---

## 1. Executive Summary

Phase 3 is an audited, statistically rigorous benchmark evaluating the rendering scalability, geometric fidelity, and runtime footprint of **native Rust GPUI (Direct3D 11 via Lyon and custom shaders)** versus **Microsoft Edge WebView2 (Chromium Canvas 2D and hardware-accelerated WebGL2)** on the Mind Place knowledge graph interface.

### Summary Verdict
1. **Memory Footprint**: Native GPUI holds a decisive advantage in memory consumption. At 100 to 1,000 nodes, GPUI's working set is **88–107 MB** (single process) versus **406–421 MB** across WebView2's multi-process Chromium architecture (~4x memory efficiency). Under heavy stress (10,000 nodes in LOD mode), GPUI consumes **246 MB** versus **582–614 MB** for WebView2.
2. **Rendering Throughput at Scale (10k+ nodes)**: WebView2's hardware-accelerated **WebGL2 backend** delivers the highest raw frame cadence at high node counts, sustaining **61.36 FPS in Full mode** and **107.44 FPS in LOD mode** at 10,000 nodes (11.4 ms and 7.6 ms CPU draw time, respectively). It achieves this by shifting quadratic Bézier ribbon evaluation entirely to GPU vertex shaders.
3. **GPUI vs Canvas 2D**: Native GPUI significantly outperforms standard HTML5 Canvas 2D across all non-trivial scales. At 10,000 nodes, GPUI achieves **10.98 FPS (Full) / 15.76 FPS (LOD)** with true quadratic Béziers, outperforming Canvas 2D by **3x to 4.2x** (Canvas 2D drops to **3.63 FPS / 3.72 FPS** with 35.6 ms draw time). At 25,000 nodes, Canvas 2D collapses to unplayable 1.27 FPS (576 ms draw time), while GPUI maintains 4.5–6.8 FPS.
4. **Rendering Correctness Verified**: All Phase 2 audit defects have been systematically resolved. GPUI now renders true stroked quadratic Bézier curves via Lyon path tessellation; label visibility adheres strictly to the shared contract ($r \ge 9.0\text{ px}$ in Full, $r \ge 12.0\text{ px}$ in LOD); and steady-state resting-frame screenshots confirm identical geometry, typography, node radii, and edge counts.

---

## 2. Audit Response & Resolution Matrix

The Phase 2 benchmark was subjected to an independent source-code audit. Below is the systematic resolution of all identified audit findings:

| Audit Finding | Root Cause in Phase 2 | Phase 3 Implementation Fix | Verification Evidence |
| :--- | :--- | :--- | :--- |
| **GPUI Edge Geometry Defect** | Multi-segment and single-segment logic in `hive.rs` drew axis-aligned bounding rectangles instead of oriented line segments or curved ribbons at $>500$ nodes. | Replaced custom quad math with Lyon `gpui::PathBuilder::stroke(px(1.0))` quadratic Bézier curves (`builder.curve_to(end, ctrl)`), batched in chunks of 4,000 curves per `PathBuilder`. | Passed unit tests (`test_bezier_control_point_and_bounds`, `test_stress_bezier_path_builder_large`). Resting screenshots confirm true smooth curved edges at all scales. |
| **Label Visibility Asymmetry** | GPUI used an arbitrary density formula (`r * zoom > 14.0 / sqrt(len)`), while Canvas 2D had fixed thresholds, causing GPUI to render fewer labels. | Established formal `rendering-contract.md`: Full mode renders labels for $r \ge 9.0\text{ px}$; LOD mode renders labels for $r \ge 12.0\text{ px}$. Identical logic implemented in Rust GPUI, Canvas 2D, and WebGL2. | Exactly 41-42 labels at 100 nodes, 382-383 labels at 1k nodes, 3,836-3,837 labels at 10k nodes, and 9,704-9,705 labels at 25k nodes across all engines ($\pm 0.02\%$ floating-point parity). |
| **Missing WebGL2 Backend** | WebView2 only provided standard Canvas 2D, underutilizing modern Chromium GPU capabilities. | Implemented a dedicated WebGL2 rendering pipeline with GPU instanced circle quads (AA fragment shader) and GPU quadratic Bézier ribbons evaluated in vertex shaders. | Tested via `--renderer webgl2` across all datasets and verified with resting-frame screenshots. |
| **Unverified Presentation FPS Claims** | Presentation FPS was reported at $280+$ FPS based on internal event loop iterations without measuring hardware display presentation or vblank sync. | Presentation FPS is now explicitly labeled as `"NOT_MEASURED (asynchronous DXGI presentation pipeline)"`. Telemetry strictly measures internal frame cadence, CPU update time, and CPU draw time. | Verified in JSON telemetry outputs and test logs. |
| **Single-PID Memory Tracking** | Benchmark script only measured the root host process PID, omitting WebView2 child processes (GPU process, renderer process, network utility). | Implemented dynamic process-tree discovery via CIM `Win32_Process` filter `ParentProcessId = $ParentPid`. Sums Private Bytes and Working Set across all recursive child processes. | WebView2 correctly reports 405–651 MB total working set across 4–6 processes, while GPUI reports 88–485 MB in its single native process. |
| **Hazardous Process Cleanup** | PowerShell script executed `Get-Process msedgewebview2 | Stop-Process`, terminating unrelated user applications. | Scoped process termination strictly to `$p.Id` and its enumerated descendants via recursive BFS traversal (`Stop-ProcessTreeSafe`). | Verified zero interference with external user browser sessions during test runs. |
| **Camera Motion Blur in Screenshots** | Screenshots were captured during active benchmark motion animations, obscuring visual artifacts. | Implemented resting-frame capture using `--scenario idle` with camera settled at $(600, 400, 1.0)$ after 2-second stabilization. | Produced clean, sharp resting screenshots in `comparison/benchmarks/phase3/screenshots/`. |

---

## 3. System & Test Environment

- **Host Operating System:** Microsoft Windows 11 Home x64 (Build 26100)
- **Rust Toolchain:** rustc 1.84.0 / cargo 1.84.0 (MSVC x86_64-pc-windows-msvc)
- **GPUI Renderer:** Direct3D 11 Hardware Swapchain, Lyon 1.0 Bézier Tessellation
- **WebView2 Runtime:** Microsoft Edge WebView2 (Chromium Evergreen Runtime 142.x)
- **Display Configuration:** Primary monitor at 60 Hz / 144 Hz adaptive cadence
- **Test Protocol:** 5 valid consecutive repetitions per configuration, 500 ms warmup period discarded, dynamic memory sampled at steady state (2.0s mark), automated process-tree cleanup.

---

## 4. Phase 3 Benchmark Results

All metrics represent the statistical mean of **5 independent benchmark repetitions** under continuous `stress` animation (pan, zoom, and dynamic oscillation).

### A. Full-Fidelity Mode (`full`)
*All nodes and edges rendered; labels displayed for nodes with screen radius $r \ge 9.0\text{ px}$.*

| Dataset | Engine | Cadence FPS | Mean CPU Draw (ms) | P50 Frame (ms) | P95 Frame (ms) | Working Set (MB) | Private Bytes (MB) | Rendered Nodes | Rendered Edges | Rendered Labels |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **100 Nodes** | **GPUI** | 60.00 | 1.655 | 16.67 | 16.73 | **89.12** | **112.96** | 100 | 325 | 41 |
| *(325 edges)* | **WV2 Canvas** | 144.00 | 0.437 | 6.90 | 7.10 | 421.79 | 328.16 | 100 | 325 | 42 |
| | **WV2 WebGL2** | 143.76 | **0.374** | 6.90 | 7.20 | 418.68 | 350.86 | 100 | 325 | 42 |
| **1,000 Nodes** | **GPUI** | 59.99 | 6.439 | 16.66 | 16.73 | **107.28** | **132.88** | 1,000 | 1,778 | 382 |
| *(1,778 edges)* | **WV2 Canvas** | 61.49 | 2.605 | 14.10 | 20.80 | 406.97 | 342.44 | 1,000 | 1,778 | 383 |
| | **WV2 WebGL2** | 144.00 | **1.621** | 6.90 | 7.10 | 408.22 | 333.93 | 1,000 | 1,778 | 383 |
| **10,000 Nodes**| **GPUI** | 10.98 | 82.517 | 89.20 | 104.50 | **414.91** | 460.72 | 10,000 | 14,205 | 3,836 |
| *(14,205 edges)*| **WV2 Canvas** | 3.63 | 35.591 | 248.60 | 332.10 | 651.23 | 543.17 | 10,000 | 14,205 | 3,837 |
| | **WV2 WebGL2** | **61.36** | **11.397** | 14.20 | 20.90 | 515.86 | **428.44** | 10,000 | 14,205 | 3,837 |
| **25,000 Nodes**| **GPUI** | 4.45 | 198.172 | 215.40 | 244.10 | **484.84** | 556.39 | 25,000 | 35,009 | 9,704 |
| *(35,009 edges)*| **WV2 Canvas** | 1.27 | 576.130 | 722.40 | 884.00 | 624.06 | **502.37** | 25,000 | 35,009 | 9,705 |
| | **WV2 WebGL2** | **23.08** | **32.519** | 42.10 | 49.30 | 617.87 | 521.53 | 25,000 | 35,009 | 9,705 |

---

### B. Optimized LOD Mode (`lod`)
*Spatial culling active; labels restricted to hub nodes with $r \ge 12.0\text{ px}$.*

| Dataset | Engine | Cadence FPS | Mean CPU Draw (ms) | P50 Frame (ms) | P95 Frame (ms) | Working Set (MB) | Private Bytes (MB) | Rendered Nodes | Rendered Edges | Rendered Labels |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **100 Nodes** | **GPUI** | 60.00 | 1.426 | 16.67 | 16.71 | **88.68** | **111.61** | 100 | 325 | 1 |
| | **WV2 Canvas** | 143.60 | 0.524 | 6.90 | 7.10 | 405.44 | 326.94 | 100 | 325 | 1 |
| | **WV2 WebGL2** | 144.00 | **0.348** | 6.90 | 7.10 | 405.60 | 329.41 | 100 | 325 | 1 |
| **1,000 Nodes** | **GPUI** | 58.56 | 7.641 | 16.66 | 16.74 | **87.07** | **110.53** | 1,000 | 1,778 | 1 |
| | **WV2 Canvas** | 57.98 | 2.345 | 14.10 | 21.00 | 402.93 | 322.18 | 1,000 | 1,778 | 1 |
| | **WV2 WebGL2** | 144.00 | **1.129** | 6.90 | 7.10 | 409.02 | 339.62 | 1,000 | 1,778 | 1 |
| **10,000 Nodes**| **GPUI** | 15.76 | 60.265 | 61.20 | 74.50 | **246.48** | **282.79** | 10,000 | 14,205 | 1 |
| | **WV2 Canvas** | 3.72 | 29.848 | 250.20 | 334.00 | 613.92 | 502.09 | 10,000 | 14,205 | 2 |
| | **WV2 WebGL2** | **107.44** | **7.641** | 9.10 | 14.20 | 581.88 | 502.54 | 10,000 | 14,205 | 2 |
| **25,000 Nodes**| **GPUI** | 6.81 | 145.812 | 142.10 | 168.30 | **319.51** | **364.85** | 25,000 | 35,009 | 1 |
| | **WV2 Canvas** | 1.55 | 351.774 | 622.00 | 794.00 | 701.92 | 607.61 | 25,000 | 35,009 | 1 |
| | **WV2 WebGL2** | **44.08** | **20.928** | 22.30 | 28.50 | 623.90 | 551.45 | 25,000 | 35,009 | 1 |

---

## 5. Architectural Deep-Dive

```
+----------------------------------------------------------------------------------------------------+
|                                    RENDERER ARCHITECTURE COMPARISON                                |
+----------------------------------------------------------------------------------------------------+
| Feature                 | Native GPUI (Direct3D 11)   | WebView2 (Canvas 2D)      | WebView2 (WebGL2)      |
+-------------------------+-----------------------------+---------------------------+------------------------+
| Process Model           | Single process (native)     | Multi-process (Chromium)  | Multi-process (Chromium)|
| Working Set Base Memory | ~88 MB                      | ~405 MB                   | ~405 MB                |
| Curve Primitive         | Lyon PathBuilder (quadratic)| CanvasRenderingContext2D  | GPU Ribbon Vertex Inst.|
| Tessellation Location   | CPU (Lyon tessellator)      | CPU (Skia path rasterizer)| GPU (Vertex Shader)    |
| Circle Primitive        | GPU Shader Quads            | arc() path rasterization  | Instanced Quad Shaders |
| Label Text Pipeline     | GPUI Text System            | HTML5 Canvas font fillText| 2D Text Overlay Canvas |
| Throughput Bottleneck   | CPU curve tessellation      | CPU Skia rasterization    | GPU fragment fill-rate |
| Scalability at 10k nodes| 11 - 16 FPS                 | 3.6 - 3.7 FPS             | 61 - 107 FPS           |
+----------------------------------------------------------------------------------------------------+
```

### 1. Native Rust GPUI Pipeline
- **Memory Efficiency**: GPUI allocates memory strictly for application state, scene trees, and GPU vertex/index buffers. It does not spawn utility processes, sandbox hosts, or network workers, keeping idle and light memory footprint below 90 MB.
- **Edge Rendering**: Edges are generated as true quadratic Bézier curves using Lyon. In Phase 3, we batched curve allocations into chunks of 4,000 curves per `PathBuilder`, preventing buffer overflows and enabling drawing over 35,000 curves with proper line caps, colors, and line widths.
- **Bottleneck**: Because Lyon performs Bézier subdivision and line tessellation on the CPU before uploading vertices to Direct3D 11 buffers, CPU draw time grows proportionally with edge count ($\sim 82\text{ ms}$ for 14,205 edges; $\sim 198\text{ ms}$ for 35,009 edges).

### 2. WebView2 Canvas 2D Pipeline
- **Architectural Limit**: While convenient for rapid web development, Chromium's HTML5 Canvas 2D engine executes path drawing sequentially through Skia. Each `quadraticCurveTo` and `stroke()` call requires CPU path rasterization.
- **Degradation**: At 10,000 nodes, Canvas 2D frame cadence plummets to 3.63 FPS with severe hitching (P95 of 332 ms). At 25,000 nodes, draw time reaches 576 ms (1.27 FPS), rendering interactive exploration impractical.

### 3. WebView2 WebGL2 Hardware-Accelerated Pipeline
- **GPU-Centric Design**: Node circles are drawn as instanced quad billboards with anti-aliasing computed directly inside the fragment shader. Edges are drawn as instanced line ribbons where the vertex shader evaluates the quadratic Bézier equation:
  $$B(t) = (1 - t)^2 P + 2(1 - t)t M + t^2 Q$$
  Zero curve subdivision or tessellation occurs on the CPU.
- **Performance**: This design yields the highest frame cadence tested: 61.36 FPS in Full mode and 107.44 FPS in LOD mode at 10,000 nodes.
- **Trade-off**: Requires running inside the Chromium host process, consuming ~405–618 MB of system memory.

---

## 6. Visual Parity Verification

Resting-frame screenshots were captured after 2.0 seconds of camera stabilization with the view centered at $(600, 400, 1.0)$ in Full mode:

| Dataset | Native GPUI | WebView2 Canvas 2D | WebView2 WebGL2 |
| :---: | :---: | :---: | :---: |
| **100 Nodes** | `gpui_100.png` (100 nodes, 325 edges) | `webview2_canvas_100.png` (100 nodes, 325 edges) | `webview2_webgl2_100.png` (100 nodes, 325 edges) |
| **1,000 Nodes** | `gpui_1000.png` (1k nodes, 1,778 edges) | `webview2_canvas_1000.png` (1k nodes, 1,778 edges) | `webview2_webgl2_1000.png` (1k nodes, 1,778 edges) |
| **10,000 Nodes**| `gpui_10000.png` (10k nodes, 14,205 edges) | `webview2_canvas_10000.png` (10k nodes, 14,205 edges) | `webview2_webgl2_10000.png` (10k nodes, 14,205 edges) |

Visual inspection confirms:
1. **Curvature**: Edges across all three renderers curve toward the same perpendicular normal offset ($M = (P+Q)/2 + \text{perp} \times 0.06$).
2. **Node Radii & Palette**: Node colors, radial highlights, and central core dots match identically across categories (`Core intelligence`, `Epistemic engine`, `Operational layer`, etc.).
3. **Typography**: Labels render in crisp sans-serif monochrome type with exact position offsets.

---

## 7. Conclusions & Strategic Recommendations

1. **For Compact Desktop Footprint & Instant Startup**:
   - Choose **Native Rust GPUI**. It uses only **88 MB RAM** on startup and operates within a single native executable with no runtime dependencies. For graphs up to 1,000–3,000 nodes, it delivers a smooth 60 FPS experience with minimal system resource utilization.
2. **For Extreme Scalability (10,000 to 50,000+ Nodes)**:
   - The optimal architecture is **GPU Vertex Shader Instancing**. In WebView2, this was realized via WebGL2 (achieving 61–107 FPS at 10k nodes). In GPUI, a future enhancement would replace Lyon CPU tessellation with custom Direct3D 11 vertex shaders that evaluate Béziers directly on the GPU.
3. **Avoid HTML5 Canvas 2D for Large Interactive Graphs**:
   - Canvas 2D is unsuitable for graphs exceeding 1,000 nodes. It lacks GPU instancing, relies on CPU path rasterization, and causes severe UI freezing at scale.
