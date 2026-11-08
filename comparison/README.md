# Mind Place Desktop Architecture Benchmark: Native GPUI vs. Microsoft Edge WebView2

A controlled, empirical desktop technology experiment comparing a **100% native Rust + GPUI** desktop implementation against a **lightweight Rust + Microsoft Edge WebView2** host for the Mind Place Personal Knowledge OS.

---

## 1. Directory Structure

```text
comparison/
├── gpui/
│   └── MindPlace-GPUI.exe         # Fully native Rust + GPUI release executable (12.8 MB)
├── webview2/
│   └── MindPlace-WebView2.exe     # Minimal native Rust host embedding WebView2 (1.0 MB)
├── benchmarks/
│   ├── results.json               # Raw empirical measurement data (JSON)
│   └── results.md                 # Detailed benchmark report & analysis
├── screenshots/
│   ├── gpui_window.png            # Visual capture of native GPUI application
│   └── webview2_window.png        # Visual capture of WebView2 application
└── README.md                      # This documentation
```

---

## 2. Deliverables & How to Run

Both applications are compiled as standalone 64-bit Windows release executables and can be launched independently.

### Running Native GPUI
```powershell
.\comparison\gpui\MindPlace-GPUI.exe
```
* **Architecture:** Single-process, GPU-accelerated immediate/retained-hybrid renderer (GPUI).
* **Dependencies:** None. Fully self-contained native Windows PE64 executable. Does not require any browser runtime, web engine, or external frameworks.

### Running WebView2
```powershell
.\comparison\webview2\MindPlace-WebView2.exe
```
* **Architecture:** Minimal Rust application host using `tao` + `wry`, embedding the Microsoft Edge WebView2 Runtime. The prototype HTML/CSS/JS is compiled directly into the binary with zero development servers or Node.js runtime.
* **Dependencies:** Requires the Microsoft Edge WebView2 Evergreen Runtime (standard on Windows 10 version 2004+ and Windows 11; located at `C:\Program Files (x86)\Microsoft\EdgeWebView\Application`).

---

## 3. How to Build from Source

### Prerequisites
* Rust toolchain (1.80+ recommended, MSVC ABI target: `x86_64-pc-windows-msvc`).
* Microsoft Edge WebView2 Evergreen Runtime.

### Rebuilding Native GPUI
```powershell
# From the repository root
cargo build --release
Copy-Item target\release\mind_place_gpui.exe comparison\gpui\MindPlace-GPUI.exe
```

### Rebuilding WebView2 Host
```powershell
# From the mind_place_webview2 crate
cd mind_place_webview2
cargo build --release
Copy-Item target\release\mind_place_webview2.exe ..\comparison\webview2\MindPlace-WebView2.exe
```

### Running the Automated Benchmark Suite
```powershell
powershell -ExecutionPolicy Bypass -File benchmark_suite.ps1
```

---

## 4. Key Metric Highlights (Preview)

| Dimension | Mind Place (Native GPUI) | Mind Place (WebView2) | Delta / Winner |
| :--- | :--- | :--- | :--- |
| **Process Count** | **1 process** | **7 processes** (Host + 6 Edge child processes) | **GPUI** (7x fewer processes) |
| **RAM (Working Set)** | **~46 MB** | **~240 - 280 MB** (tree sum) | **GPUI** (5.5x - 6x less RAM) |
| **RAM (Private Bytes)** | **~38 MB** | **~195 - 225 MB** (tree sum) | **GPUI** (5.5x less private memory) |
| **Startup Time (Warm)** | **~600 - 700 ms** | **~1100 - 1500 ms** | **GPUI** (~2x faster launch) |
| **Host Binary Size** | 12.8 MB (all-inclusive) | 1.0 MB (+ 450 MB shared runtime) | **Trade-off** (Host size vs Runtime dependency) |
| **UI Rendering Engine** | Direct3D / Vulkan + DirectWrite | Chromium Blink + V8 + Skia | **GPUI** native frame pacing |

*Full data and deep-dive analysis are available in [`comparison/benchmarks/results.md`](file:///d:/projects/gemini-rust-gpui-test/comparison/benchmarks/results.md).*
