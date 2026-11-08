# Phase 3 Audit Findings & Remediation Plan

This document maps each audit finding from Phase 2 directly to its root cause and the exact remediation implemented in Phase 3.

---

## 1. GPUI Geometry Defect: Axis-Aligned Box Edges

| Item | Description |
| :--- | :--- |
| **Audit Finding** | `hive.rs` used `paint_quad` with bounding boxes for edges: `let segs = if nodes.len() > 3000 { 1 } ...`. Large graphs rendered rectangular bounding boxes instead of real lines or curves. |
| **Root Cause** | GPUI's `paint_quad` only supports axis-aligned rectangles. Stretched across $(ax, ay)$ to $(bx, by)$, it created filled boxes. |
| **Remediation** | Implemented true stroked quadratic Bézier curves using `gpui::PathBuilder::stroke(px(width))`. Lyon tessellates the Bézier ribbon into GPU-ready triangle vertices. Added support for dashed strokes (`dash_array(&[px(3.0), px(5.0)])`) and batching across edges. Verified via automated test `test_path_builder_bezier_stroke`. |

---

## 2. Shared Label Visibility Mismatch

| Item | Description |
| :--- | :--- |
| **Audit Finding** | GPUI culled labels at $r < 14$, drawing only ~30 labels at 1k nodes. WebView2 had threshold $r \ge 9$, drawing >900 labels. |
| **Root Cause** | Inconsistent LOD thresholds between Rust and HTML/JS created an unfair CPU font shaping workload comparison. |
| **Remediation** | Established an explicit shared label contract (`rendering-contract.md`). Unified thresholds across all renderers. Added real-time per-frame telemetry counters (`labels_rendered`) to verify both engines draw identical label counts. |

---

## 3. Filtering & Dimming Disparity

| Item | Description |
| :--- | :--- |
| **Audit Finding** | GPUI skipped non-matching category nodes entirely (`continue`), while WebView2 Canvas 2D rendered them in a dimmed state. |
| **Root Cause** | Divergent visibility semantics between the two codebases. |
| **Remediation** | Standardized filtering rules: category filter culls non-matching nodes from the active graph identically in all three renderers. Search query matches highlight matching nodes and dim non-matching nodes consistently. |

---

## 4. Benchmark Harness Flaws

| Item | Description |
| :--- | :--- |
| **Audit Finding 1** | WebView2 measured a separate `requestAnimationFrame` benchmark loop disconnected from `drawHive()`. |
| **Remediation 1** | Wrapped `drawHive()` execution directly with `performance.now()` telemetry, measuring exact CPU draw time, update time, and frame interval. |
| **Audit Finding 2** | Hardcoded `ProcessTreeCount = if ($AppType -eq "WebView2") { 6 } else { 1 }`. |
| **Remediation 2** | Implemented dynamic recursive process discovery using Win32 CIM instances (`Get-CimInstance Win32_Process`) that records actual live child PIDs and states. |
| **Audit Finding 3** | Destructive `Stop-Process -Name msedgewebview2* -Force` killed unrelated system processes. |
| **Remediation 3** | Replaced with safe process-tree termination restricted strictly to child PIDs descended from the benchmark-launched `$rootPid`. |
| **Audit Finding 4** | Universal 20 ms dropped-frame threshold without hardware refresh rate context. |
| **Remediation 4** | Switched to explicit display refresh timing budgets (144 Hz = 6.94 ms, 60 Hz = 16.67 ms, hitch = 33.33 ms). |

---

## 5. Dataset Determinism & Reproducibility

| Item | Description |
| :--- | :--- |
| **Audit Finding** | `generate_datasets.rs` iterated over `HashSet`, causing non-deterministic edge serialization ordering. Undirected degree calculation was imprecise. |
| **Remediation** | Canonicalized edge tuples (`(min(u, v), max(u, v))`), sorted nodes and edges deterministically before serialization, documented $2E/N$ undirected average degree, and generated verifiable SHA-256 dataset manifests. |

---

## 6. Addition of WebGL2 Backend

| Item | Description |
| :--- | :--- |
| **Audit Finding** | Phase 2 only compared GPUI with Canvas 2D. Canvas 2D is inherently bottlenecked by single-threaded CPU path rasterization and Chromium IPC. |
| **Remediation** | Implemented a dedicated hardware-accelerated **WebGL2** rendering pipeline in `MindPlace-WebView2.exe`, featuring GPU-buffered instanced node rendering and GPU curve tessellation. Kept Canvas 2D selectable via `--renderer canvas` vs `--renderer webgl2`. |
