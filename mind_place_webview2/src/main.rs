use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder};

static HTML_CONTENT: &str = include_str!("../assets/index.html");

#[derive(Debug)]
enum UserEvent {
    IpcMessage(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut benchmark_file: Option<String> = None;
    let mut scenario: String = "stress".to_string();
    let mut duration_sec: f64 = 10.0;
    let mut output_file: Option<String> = None;
    let mut renderer: String = "canvas".to_string();
    let mut render_mode: String = "full".to_string();
    let mut target_fps: f64 = 60.0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--benchmark" => {
                if i + 1 < args.len() {
                    benchmark_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--scenario" => {
                if i + 1 < args.len() {
                    scenario = args[i + 1].clone();
                    i += 1;
                }
            }
            "--duration" => {
                if i + 1 < args.len() {
                    duration_sec = args[i + 1].parse().unwrap_or(10.0);
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--renderer" => {
                if i + 1 < args.len() {
                    renderer = args[i + 1].to_lowercase();
                    i += 1;
                }
            }
            "--mode" => {
                if i + 1 < args.len() {
                    render_mode = args[i + 1].to_lowercase();
                    i += 1;
                }
            }
            "--target-fps" => {
                if i + 1 < args.len() {
                    target_fps = args[i + 1].parse().unwrap_or(60.0);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Mind Place — Personal Knowledge OS & Shared-Brain Archive (WebView2)")
        .with_inner_size(LogicalSize::new(1440.0, 900.0))
        .build(&event_loop)?;

    let proxy_clone = proxy.clone();
    let webview = WebViewBuilder::new()
        .with_html(HTML_CONTENT)
        .with_ipc_handler(move |req: Request<String>| {
            let msg = req.body().clone();
            let _ = proxy_clone.send_event(UserEvent::IpcMessage(msg));
        })
        .build(&window)?;

    let mut benchmark_started = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::IpcMessage(msg)) => {
                println!("[WebView2 IPC Event] {}", if msg.len() > 100 { &msg[..100] } else { &msg });
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&msg) {
                    let msg_type = parsed.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    if msg_type == "page_ready" {
                        // Apply initial renderer backend and render mode
                        let init_script = format!(
                            r#"
                            (function() {{
                                if (window.setRendererBackend) window.setRendererBackend('{renderer}');
                                if (window.setRenderMode) window.setRenderMode('{render_mode}');
                            }})();
                            "#,
                            renderer = renderer,
                            render_mode = render_mode
                        );
                        let _ = webview.evaluate_script(&init_script);

                        if !benchmark_started {
                            if let Some(ref path) = benchmark_file {
                                benchmark_started = true;
                                println!("[WebView2] Benchmark mode activated for: {} (renderer: {}, mode: {}, target_fps: {})", path, renderer, render_mode, target_fps);
                                let t_load_start = Instant::now();
                                match fs::read_to_string(path) {
                                    Ok(ds_content) => {
                                        let t_load_ms =
                                            t_load_start.elapsed().as_secs_f64() * 1000.0;
                                        println!(
                                            "[WebView2] Dataset read in {:.2} ms ({} bytes)",
                                            t_load_ms,
                                            ds_content.len()
                                        );

                                        let script = format!(
                                            r#"
                                            (function() {{
                                                try {{
                                                    if (window.setRendererBackend) window.setRendererBackend('{renderer}');
                                                    if (window.setRenderMode) window.setRenderMode('{render_mode}');
                                                    const rawStr = {:?};
                                                    const ds = JSON.parse(rawStr);
                                                    const tLoad = {:.2};
                                                    window.loadHiveDataset(ds);
                                                    window.runGraphBenchmark('{scenario}', {duration_sec:.1}, {target_fps:.1}, function(res) {{
                                                        res.dataset_load_time_ms = tLoad;
                                                        sendNativeMessage({{ type: 'benchmark_result', data: res }});
                                                    }});
                                                }} catch (e) {{
                                                    sendNativeMessage({{ type: 'benchmark_error', error: (e && e.stack) ? e.stack : String(e) }});
                                                }}
                                            }})();
                                            "#,
                                            ds_content,
                                            t_load_ms,
                                            renderer = renderer,
                                            render_mode = render_mode,
                                            scenario = scenario,
                                            duration_sec = duration_sec,
                                            target_fps = target_fps
                                        );

                                        if let Err(e) = webview.evaluate_script(&script) {
                                            eprintln!(
                                                "[WebView2] Failed to evaluate benchmark script: {}",
                                                e
                                            );
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!(
                                            "[WebView2] Failed to read dataset '{}': {}",
                                            path, err
                                        );
                                        *control_flow = ControlFlow::Exit;
                                    }
                                }
                            }
                        }
                    } else if msg_type == "benchmark_result" {
                        let data = &parsed["data"];
                        println!("BENCHMARK_RESULT_JSON:{}", data);
                        if let Some(ref out_path) = output_file {
                            let _ = fs::write(
                                out_path,
                                serde_json::to_string_pretty(data).unwrap_or_default(),
                            );
                        }
                        *control_flow = ControlFlow::Exit;
                    } else if msg_type == "benchmark_error" {
                        eprintln!("[WebView2 JS Error] {}", parsed["error"]);
                        *control_flow = ControlFlow::Exit;
                    } else if msg_type == "load_dataset_request" {
                        if let Some(size) = parsed.get("size").and_then(|s| s.as_u64()) {
                            let p3_path = format!(
                                "comparison/benchmarks/phase3/datasets/graph_{}.json",
                                size
                            );
                            let legacy_path = format!(
                                "comparison/benchmarks/graph-stress/datasets/graph_{}.json",
                                size
                            );
                            let target_path = if Path::new(&p3_path).exists() {
                                p3_path
                            } else {
                                legacy_path
                            };

                            if Path::new(&target_path).exists() {
                                if let Ok(content) = fs::read_to_string(&target_path) {
                                    let script = format!(
                                        r#"
                                        (function() {{
                                            const raw = {:?};
                                            window.loadHiveDataset(JSON.parse(raw));
                                        }})();
                                        "#,
                                        content
                                    );
                                    let _ = webview.evaluate_script(&script);
                                }
                            }
                        }
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
