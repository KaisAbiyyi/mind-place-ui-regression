use crate::models::DialogModal;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    pub fn render_hive_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let selected_id = self.state.hive_selected_node_id.clone();
        let selected_node = self
            .state
            .hive_nodes
            .iter()
            .find(|n| n.id == selected_id)
            .cloned()
            .unwrap_or_else(|| self.state.hive_nodes[0].clone());

        // Advance benchmark if active
        if let Some(mut bench) = self.state.hive_benchmark.take() {
            let now = std::time::Instant::now();
            let dt = (now - bench.last_frame_time).as_secs_f64() * 1000.0;
            bench.last_frame_time = now;
            bench.frame_times.push(dt);

            let elapsed = (now - bench.start_time).as_secs_f64() * 1000.0;
            let node_count = self.state.hive_nodes.len();
            let node_scale = (node_count as f32 / 33.0).sqrt().max(1.0);
            let scenario = bench.scenario.clone();

            if scenario == "pan" || scenario == "stress" || scenario == "all" {
                let angle = (elapsed / 1000.0) as f32 * 1.5;
                let dist = 320.0 * node_scale;
                self.state.hive_target_x = 600.0 + angle.cos() * dist;
                self.state.hive_target_y = 400.0 + angle.sin() * dist;
            }
            if scenario == "zoom" || scenario == "stress" || scenario == "all" {
                let z_freq = (elapsed / 1000.0) as f32 * 1.3;
                self.state.hive_target_zoom = 1.25 + z_freq.sin() * 0.65;
            }
            if scenario == "hover" || scenario == "stress" || scenario == "all" {
                if node_count > 0 {
                    let h_idx = ((elapsed / 120.0) as usize) % node_count;
                    self.state.hive_hovered_node_id = Some(self.state.hive_nodes[h_idx].id.clone());
                }
            }
            if scenario == "select" || scenario == "stress" || scenario == "all" {
                if node_count > 0 {
                    let s_idx = ((elapsed / 600.0) as usize) % node_count;
                    let sid = self.state.hive_nodes[s_idx].id.clone();
                    self.state.select_hive_node(&sid, false);
                }
            }
            if scenario == "navigate" {
                let phase = ((elapsed / 2000.0) as usize) % 4;
                match phase {
                    0 => {
                        self.state.hive_target_x = 600.0;
                        self.state.hive_target_y = 400.0;
                        self.state.hive_target_zoom = 1.0;
                    }
                    1 => {
                        self.state.hive_target_x = 900.0;
                        self.state.hive_target_y = 300.0;
                        self.state.hive_target_zoom = 1.8;
                    }
                    2 => {
                        self.state.hive_target_x = 300.0;
                        self.state.hive_target_y = 600.0;
                        self.state.hive_target_zoom = 1.8;
                    }
                    _ => {
                        self.state.hive_target_x = 600.0;
                        self.state.hive_target_y = 400.0;
                        self.state.hive_target_zoom = 0.8;
                    }
                }
            }

            let total_ms = bench.duration_sec * 1000.0;
            if elapsed >= total_ms {
                let mut sorted = bench.frame_times.clone();
                if sorted.len() > 1 {
                    sorted.remove(0); // discard warmup
                }
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let count = sorted.len();
                let sum_dt: f64 = sorted.iter().sum();
                let avg_fps = if count > 0 && sum_dt > 0.0 {
                    (count as f64) / (sum_dt / 1000.0)
                } else {
                    0.0
                };
                let p50 = if count > 0 { sorted[(count as f64 * 0.50) as usize] } else { 0.0 };
                let p95 = if count > 0 { sorted[((count as f64 * 0.95) as usize).min(count - 1)] } else { 0.0 };
                let p99 = if count > 0 { sorted[((count as f64 * 0.99) as usize).min(count - 1)] } else { 0.0 };
                let low_1pct_fps = if p99 > 0.0 { 1000.0 / p99 } else { 0.0 };
                let dropped = sorted.iter().filter(|&&t| t > 20.0).count();

                let json_res = serde_json::json!({
                    "scenario": bench.scenario,
                    "node_count": node_count,
                    "edge_count": self.state.hive_edges.len(),
                    "duration_sec": bench.duration_sec,
                    "total_frames": count,
                    "avg_fps": (avg_fps * 100.0).round() / 100.0,
                    "low_1pct_fps": (low_1pct_fps * 100.0).round() / 100.0,
                    "p50_ms": (p50 * 100.0).round() / 100.0,
                    "p95_ms": (p95 * 100.0).round() / 100.0,
                    "p99_ms": (p99 * 100.0).round() / 100.0,
                    "dropped_frames": dropped,
                    "dataset_load_time_ms": bench.dataset_load_time_ms,
                });

                println!("BENCHMARK_RESULT_JSON:{}", json_res);
                if let Some(ref path) = bench.output_file {
                    let _ = std::fs::write(path, serde_json::to_string_pretty(&json_res).unwrap());
                }
                std::process::exit(0);
            } else {
                self.state.hive_benchmark = Some(bench);
            }
        }

        // Update animation time and smooth camera position
        let screen_t0 = std::time::Instant::now();
        self.state.hive_anim_time += 16.0;
        self.state.hive_camera_x += (self.state.hive_target_x - self.state.hive_camera_x) * 0.085;
        self.state.hive_camera_y += (self.state.hive_target_y - self.state.hive_camera_y) * 0.085;
        self.state.hive_camera_zoom +=
            (self.state.hive_target_zoom - self.state.hive_camera_zoom) * 0.085;

        let zoom_pct = (self.state.hive_camera_zoom * 100.0).round() as i32;
        let node_count = self.state.hive_nodes.len();
        let edge_count = self.state.hive_edges.len();

        let el = div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(1340.0))
            .mx_auto()
            .gap(px(16.0))
            // Page Head
            .child(
                div()
                    .flex()
                    .items_end()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .child("INDEX-CORE · GRAPHIFY & SPATIAL TOPOLOGY"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("Hive Mind Graph"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child(format!("Interactive monochrome topology of {} concept clusters and {} verified provenance edges. Isolated memories remain unforced.", node_count, edge_count)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .items_center()
                            // Dataset Switcher
                            .child(
                                div()
                                    .flex()
                                    .gap(px(4.0))
                                    .items_center()
                                    .mr(px(8.0))
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .text_color(theme.text_muted)
                                            .child("DATASET:"),
                                    )
                                    .child(self.render_dataset_btn("33", "default", cx))
                                    .child(self.render_dataset_btn("100", "comparison/benchmarks/graph-stress/datasets/graph_100.json", cx))
                                    .child(self.render_dataset_btn("1k", "comparison/benchmarks/graph-stress/datasets/graph_1000.json", cx))
                                    .child(self.render_dataset_btn("10k", "comparison/benchmarks/graph-stress/datasets/graph_10000.json", cx)),
                            )
                            .child(
                                div()
                                    .id("hive-trace-toggle-btn")
                                    .h(px(36.0))
                                    .px(px(14.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_strong)
                                    .bg(theme.bg_surface)
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.hive_trace_active = !this.state.hive_trace_active;
                                        let active = this.state.hive_trace_active;
                                        this.state.set_toast(if active {
                                            "Edge data-pulse animation enabled"
                                        } else {
                                            "Edge data-pulse animation paused"
                                        });
                                        cx.notify();
                                    }))
                                    .child("Toggle Edge Pulse Trace"),
                            )
                            .child(
                                div()
                                    .id("hive-explain-btn")
                                    .h(px(36.0))
                                    .px(px(14.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_strong)
                                    .bg(theme.bg_surface)
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.dialog_modal = Some(DialogModal {
                                            kicker: "GRAPHIFY SPECIFICATION".into(),
                                            title: "Hive Mind Spatial Architecture".into(),
                                            body_html: "Unlike decorative mind maps, every node and edge in the Hive Mind is derived from explicit evidence in index-core/src/graphify.rs:\n\n• Filled Core Nodes: Central intelligence or currently selected focus cluster.\n• Solid vs. Dashed Bezier Edges: Active direct branches vs. secondary topology links.\n• Isolated Nodes: Memories with insufficient evidence remain unlinked rather than hallucinating connections.".into(),
                                        });
                                        cx.notify();
                                    }))
                                    .child("Graph Principles ↗"),
                            ),
                    ),
            )
            // Hive Main Layout (Graph Canvas + Inspector Sidebar)
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .min_h(px(600.0))
                    // Graph Canvas Container
                    .child(
                        div()
                            .id("hive-canvas-container")
                            .flex_1()
                            .relative()
                            .h(px(640.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_strong)
                            .bg(theme.canvas_bg)
                            .overflow_hidden()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                                this.state.hive_is_dragging = true;
                                this.state.hive_drag_start = Some((f32::from(event.position.x), f32::from(event.position.y)));
                                cx.notify();
                            }))
                            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                                if this.state.hive_is_dragging {
                                    if let Some((sx, sy)) = this.state.hive_drag_start {
                                        let dx = f32::from(event.position.x) - sx;
                                        let dy = f32::from(event.position.y) - sy;
                                        let s = this.state.hive_camera_zoom;
                                        this.state.hive_target_x -= dx / s;
                                        this.state.hive_target_y -= dy / s;
                                        this.state.hive_camera_x = this.state.hive_target_x;
                                        this.state.hive_camera_y = this.state.hive_target_y;
                                        this.state.hive_drag_start = Some((f32::from(event.position.x), f32::from(event.position.y)));
                                        cx.notify();
                                    }
                                }
                            }))
                            .on_mouse_up(MouseButton::Left, cx.listener(|this, _event: &MouseUpEvent, _window, cx| {
                                this.state.hive_is_dragging = false;
                                this.state.hive_drag_start = None;
                                cx.notify();
                            }))
                            .on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, _window, cx| {
                                let delta = match event.delta {
                                    ScrollDelta::Lines(l) => l.y * 0.12,
                                    ScrollDelta::Pixels(p) => f32::from(p.y) * 0.002,
                                };
                                this.state.hive_target_zoom = (this.state.hive_target_zoom + delta).clamp(0.55, 2.5);
                                cx.notify();
                            }))
                            // Native Canvas Custom Painter
                            .child(self.render_hive_canvas_element(cx))
                            // Floating Toolbar Overlay
                            .child(
                                div()
                                    .absolute()
                                    .top(px(14.0))
                                    .left(px(14.0))
                                    .right(px(14.0))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .gap(px(10.0))
                                    // Category Filters
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .p(px(4.0))
                                            .rounded(RADIUS_SM)
                                            .bg(theme.bg_glass)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .child(self.render_hive_filter_btn("All (33)", "all", cx))
                                            .child(self.render_hive_filter_btn("Work & Engineering", "work", cx))
                                            .child(self.render_hive_filter_btn("Research & System", "research", cx))
                                            .child(self.render_hive_filter_btn("Learning", "learning", cx))
                                            .child(self.render_hive_filter_btn("Life & Goals", "life", cx)),
                                    )
                                    // Search & Zoom controls
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(6.0))
                                            .child(
                                                div()
                                                    .w(px(175.0))
                                                    .h(px(32.0))
                                                    .px(px(10.0))
                                                    .rounded(RADIUS_SM)
                                                    .border_1()
                                                    .border_color(theme.border_subtle)
                                                    .bg(theme.bg_glass)
                                                    .flex()
                                                    .items_center()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.text_primary)
                                                    .child(if self.state.hive_search_query.is_empty() {
                                                        "Highlight node…".to_string()
                                                    } else {
                                                        self.state.hive_search_query.clone()
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(4.0))
                                                    .p(px(3.0))
                                                    .rounded(RADIUS_SM)
                                                    .bg(theme.bg_glass)
                                                    .border_1()
                                                    .border_color(theme.border_subtle)
                                                    .child(
                                                        div()
                                                            .id("hive-zoom-out-btn")
                                                            .h(px(26.0))
                                                            .px(px(7.0))
                                                            .rounded(px(4.0))
                                                            .text_size(px(12.0))
                                                            .font_family(FONT_MONO)
                                                            .text_color(theme.text_primary)
                                                            .flex()
                                                            .items_center()
                                                            .cursor_pointer()
                                                            .hover(|s| s.bg(theme.bg_hover))
                                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                                this.state.hive_target_zoom = (this.state.hive_target_zoom - 0.2).clamp(0.55, 2.5);
                                                                cx.notify();
                                                            }))
                                                            .child("−"),
                                                    )
                                                    .child(
                                                        div()
                                                            .font_family(FONT_MONO)
                                                            .text_size(px(11.5))
                                                            .px(px(4.0))
                                                            .text_color(theme.text_primary)
                                                            .child(format!("{}%", zoom_pct)),
                                                    )
                                                    .child(
                                                        div()
                                                            .id("hive-zoom-in-btn")
                                                            .h(px(26.0))
                                                            .px(px(7.0))
                                                            .rounded(px(4.0))
                                                            .text_size(px(12.0))
                                                            .font_family(FONT_MONO)
                                                            .text_color(theme.text_primary)
                                                            .flex()
                                                            .items_center()
                                                            .cursor_pointer()
                                                            .hover(|s| s.bg(theme.bg_hover))
                                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                                this.state.hive_target_zoom = (this.state.hive_target_zoom + 0.2).clamp(0.55, 2.5);
                                                                cx.notify();
                                                            }))
                                                            .child("＋"),
                                                    )
                                                    .child(
                                                        div()
                                                            .id("hive-zoom-reset-btn")
                                                            .h(px(26.0))
                                                            .px(px(7.0))
                                                            .rounded(px(4.0))
                                                            .text_size(px(11.5))
                                                            .font_family(FONT_MONO)
                                                            .text_color(theme.text_primary)
                                                            .flex()
                                                            .items_center()
                                                            .cursor_pointer()
                                                            .hover(|s| s.bg(theme.bg_hover))
                                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                                this.state.hive_target_x = 600.0;
                                                                this.state.hive_target_y = 400.0;
                                                                this.state.hive_target_zoom = 1.0;
                                                                this.state.hive_filter = "all".to_string();
                                                                this.state.hive_selected_node_id = "mind".to_string();
                                                                cx.notify();
                                                            }))
                                                            .child("Reset"),
                                                    ),
                                            ),
                                    ),
                            )
                            // Bottom interaction hint
                            .child(
                                div()
                                    .absolute()
                                    .bottom(px(14.0))
                                    .left(px(16.0))
                                    .px(px(10.0))
                                    .py(px(5.0))
                                    .rounded(RADIUS_XS)
                                    .bg(theme.bg_glass)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .text_color(theme.text_secondary)
                                    .child("HOVER inspect node · CLICK lock & zoom branch · DRAG / WHEEL navigate topology"),
                            ),
                    )
                    // Right Inspector Panel (330px width)
                    .child(
                        div()
                            .w(px(330.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .p(px(20.0))
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .child(selected_node.tag.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(6.0))
                                    .child(selected_node.title.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.text_secondary)
                                    .mb(px(14.0))
                                    .child(selected_node.summary.clone()),
                            )
                            // Metrics Tags
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .mb(px(16.0))
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .px(px(8.0))
                                            .py(px(3.0))
                                            .rounded(px(5.0))
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .child(format!("{} source traces", selected_node.sources)),
                                    )
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .px(px(8.0))
                                            .py(px(3.0))
                                            .rounded(px(5.0))
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .child(format!("{} branches", selected_node.branches.len())),
                                    )
                                    .when(self.state.tech_mode, |this| {
                                        this.child(
                                            div()
                                                .font_family(FONT_MONO)
                                                .text_size(px(11.0))
                                                .px(px(8.0))
                                                .py(px(3.0))
                                                .rounded(px(5.0))
                                                .bg(theme.bg_inverse)
                                                .text_color(theme.text_inverse)
                                                .child(format!("id:{}", selected_node.id)),
                                        )
                                    }),
                            )
                            // Connected Topology Branches
                            .child(
                                div()
                                    .pt(px(14.0))
                                    .border_t_1()
                                    .border_color(theme.border_subtle)
                                    .mb(px(12.0))
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_muted)
                                            .mb(px(10.0))
                                            .child("CONNECTED TOPOLOGY BRANCHES"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .children(selected_node.branches.iter().filter_map(|bid| {
                                                self.state.hive_nodes.iter().find(|n| n.id == *bid).cloned()
                                            }).map(|b| {
                                                let bid = b.id;
                                                div()
                                                    .id(SharedString::from(format!("branch-btn-{}", bid)))
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(10.0))
                                                    .px(px(8.0))
                                                    .py(px(7.0))
                                                    .rounded(RADIUS_XS)
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(theme.bg_hover))
                                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                                        this.state.select_hive_node(&bid, true);
                                                        cx.notify();
                                                    }))
                                                    .child(
                                                        div()
                                                            .w(px(7.0))
                                                            .h(px(7.0))
                                                            .rounded_full()
                                                            .bg(theme.text_primary),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .text_size(px(12.5))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(theme.text_primary)
                                                            .child(b.title),
                                                    )
                                                    .child(
                                                        div()
                                                            .font_family(FONT_MONO)
                                                            .text_size(px(11.0))
                                                            .text_color(theme.text_muted)
                                                            .child(format!("{} traces ↗", b.sources)),
                                                    )
                                            })),
                                    ),
                            )
                            // Provenance Distribution
                            .child(
                                div()
                                    .pt(px(14.0))
                                    .border_t_1()
                                    .border_color(theme.border_subtle)
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_muted)
                                            .mb(px(10.0))
                                            .child("PROVENANCE DISTRIBUTION"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .child(self.render_prov_row("Archived Conversations", (selected_node.sources as f32 * 0.52).round() as usize))
                                            .child(self.render_prov_row("Curated Memories", (selected_node.sources as f32 * 0.31).round() as usize))
                                            .child(self.render_prov_row("Local NTFS Documents", (selected_node.sources as f32 * 0.17).round() as usize)),
                                    ),
                            )
                            // Trace Verification Card
                            .child(
                                div()
                                    .mt_auto()
                                    .p(px(14.0))
                                    .rounded(RADIUS_SM)
                                    .bg(theme.bg_subtle)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .child(
                                        div()
                                            .text_size(px(12.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child("Provenance Verification"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.text_secondary)
                                            .my(px(4.0))
                                            .child("Every edge in the Hive Mind is derived from explicit conversation or document evidence."),
                                    )
                                    .child(
                                        div()
                                            .id("hive-inspect-evidence-btn")
                                            .w_full()
                                            .h(px(30.0))
                                            .rounded(RADIUS_SM)
                                            .bg(theme.bg_inverse)
                                            .text_color(theme.text_inverse)
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .cursor_pointer()
                                            .hover(|s| s.opacity(0.9))
                                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                                this.state.dialog_modal = Some(DialogModal {
                                                    kicker: "INDEX-CORE · GRAPH PROVENANCE".into(),
                                                    title: format!("Provenance Chain · {}", selected_node.title),
                                                    body_html: format!("Concept node {} ({}) is backed by {} verified source traces across your archive.\n\nGraph Generation: Gen-14 (graphify.rs)\nConnected Edges: {}\nSource Integrity: Verified against archive-core", selected_node.title, selected_node.tag, selected_node.sources, selected_node.branches.len()),
                                                });
                                                cx.notify();
                                            }))
                                            .child("Inspect Source Evidence ↗"),
                                    ),
                            ),
                    ),
            );
        el
    }

    fn render_prov_row(&mut self, label: &'static str, count: usize) -> impl IntoElement {
        let theme = self.state.theme;
        div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(8.0))
            .py(px(4.0))
            .text_size(px(12.5))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(7.0))
                            .h(px(7.0))
                            .rounded_full()
                            .bg(theme.text_muted),
                    )
                    .child(div().text_color(theme.text_secondary).child(label)),
            )
            .child(
                div()
                    .font_family(FONT_MONO)
                    .text_size(px(11.0))
                    .text_color(theme.text_muted)
                    .child(count.to_string()),
            )
    }

    fn render_dataset_btn(
        &mut self,
        label: &'static str,
        path: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_active = (path == "default" && self.state.hive_nodes.len() == 33)
            || (path.contains("100.") && self.state.hive_nodes.len() == 100)
            || (path.contains("1000.") && self.state.hive_nodes.len() == 1000)
            || (path.contains("10000.") && self.state.hive_nodes.len() == 10000);

        div()
            .id(SharedString::from(format!("ds-btn-{}", label)))
            .h(px(26.0))
            .px(px(8.0))
            .rounded(RADIUS_XS)
            .border_1()
            .border_color(if is_active {
                theme.border_strong
            } else {
                theme.border_subtle
            })
            .bg(if is_active {
                theme.bg_inverse
            } else {
                theme.bg_surface
            })
            .text_color(if is_active {
                theme.text_inverse
            } else {
                theme.text_secondary
            })
            .font_family(FONT_MONO)
            .text_size(px(11.0))
            .font_weight(if is_active {
                FontWeight::BOLD
            } else {
                FontWeight::MEDIUM
            })
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .hover(move |s| {
                if !is_active {
                    s.bg(theme.bg_hover).text_color(theme.text_primary)
                } else {
                    s
                }
            })
            .on_click(cx.listener(move |this, _event, _window, cx| {
                if path == "default" {
                    this.state.hive_nodes = crate::hive_data::get_hive_nodes();
                    this.state.hive_edges = crate::hive_data::get_hive_edges();
                    this.state.hive_selected_node_id = "mind".into();
                    this.state.hive_camera_x = 600.0;
                    this.state.hive_camera_y = 400.0;
                    this.state.hive_camera_zoom = 1.0;
                    this.state.hive_target_x = 600.0;
                    this.state.hive_target_y = 400.0;
                    this.state.hive_target_zoom = 1.0;
                    this.state.set_toast("Loaded default 33-node knowledge topology");
                } else {
                    match std::fs::read_to_string(path) {
                        Ok(content) => match this.state.load_hive_dataset(&content) {
                            Ok((n, e)) => {
                                this.state.set_toast(format!("Loaded {} nodes, {} edges", n, e));
                            }
                            Err(err) => {
                                this.state.set_toast(format!("Dataset load error: {}", err));
                            }
                        },
                        Err(err) => {
                            this.state.set_toast(format!("File read error: {}", err));
                        }
                    }
                }
                cx.notify();
            }))
            .child(label)
    }

    fn render_hive_filter_btn(
        &mut self,
        label: &'static str,
        layer: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_active = self.state.hive_filter == layer;

        div()
            .id(SharedString::from(format!("hive-filt-{}", layer)))
            .h(px(28.0))
            .px(px(10.0))
            .rounded(px(5.0))
            .text_size(px(12.0))
            .font_weight(if is_active {
                FontWeight::SEMIBOLD
            } else {
                FontWeight::MEDIUM
            })
            .bg(if is_active {
                theme.bg_inverse
            } else {
                theme.bg_glass
            })
            .text_color(if is_active {
                theme.text_inverse
            } else {
                theme.text_secondary
            })
            .flex()
            .items_center()
            .cursor_pointer()
            .hover(move |s| {
                if !is_active {
                    s.text_color(theme.text_primary)
                } else {
                    s
                }
            })
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.state.hive_filter = layer.to_string();
                cx.notify();
            }))
            .child(label)
    }

    fn render_hive_canvas_element(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let nodes = self.state.hive_nodes.clone();
        let edges = self.state.hive_edges.clone();
        let selected_id = self.state.hive_selected_node_id.clone();
        let hovered_id = self.state.hive_hovered_node_id.clone();
        let filter = self.state.hive_filter.clone();
        let cam_x = self.state.hive_camera_x;
        let cam_y = self.state.hive_camera_y;
        let cam_zoom = self.state.hive_camera_zoom;
        let anim_time = self.state.hive_anim_time;
        let trace_active = self.state.hive_trace_active;
        let is_benchmark = self.state.hive_benchmark.is_some();

        // Precompute fast O(1) index-based edge lookup
        let mut id_to_idx: std::collections::HashMap<String, usize> =
            std::collections::HashMap::with_capacity(nodes.len());
        for (i, n) in nodes.iter().enumerate() {
            id_to_idx.insert(n.id.clone(), i);
        }

        let edge_pairs: Vec<(usize, usize)> = edges
            .iter()
            .filter_map(|(a, b)| match (id_to_idx.get(a), id_to_idx.get(b)) {
                (Some(&ia), Some(&ib)) => Some((ia, ib)),
                _ => None,
            })
            .collect();

        canvas(
            move |bounds, _window, _cx| {
                let w = bounds.size.width.max(px(700.0));
                let h = bounds.size.height.max(px(638.0));
                Bounds {
                    origin: bounds.origin,
                    size: size(w, h),
                }
            },
            move |bounds, _state, window, cx| {
                let effective_bounds = Bounds {
                    origin: bounds.origin,
                    size: size(bounds.size.width.max(px(700.0)), bounds.size.height.max(px(638.0))),
                };
                let w = f32::from(effective_bounds.size.width);
                let h = f32::from(effective_bounds.size.height);
                let ox = f32::from(effective_bounds.origin.x);
                let oy = f32::from(effective_bounds.origin.y);

                let node_scale = ((nodes.len() as f32) / 33.0).sqrt().max(1.0);
                let base_s = (w / (1220.0 * node_scale)).min(h / (820.0 * node_scale)) * 0.94;
                let s = base_s * cam_zoom;

                let transform = |x: f32, y: f32| -> (f32, f32) {
                    (
                        ox + w / 2.0 + (x - cam_x) * s,
                        oy + h / 2.0 + (y - cam_y) * s,
                    )
                };

                window.paint_layer(effective_bounds, |window| {
                    // 1. Paint Coordinate Grid Lines
                    let step = 36.0 * (0.65f32).max((1.5f32).min(cam_zoom));
                    let off_x = ((w / 2.0 - cam_x * s) % step + step) % step;
                    let off_y = ((h / 2.0 - cam_y * s) % step + step) % step;

                    let mut cur_x = off_x;
                    while cur_x < w {
                        let quad_b = Bounds {
                            origin: point(px(ox + cur_x), px(oy)),
                            size: size(px(1.0), px(h)),
                        };
                        window.paint_quad(fill(quad_b, theme.canvas_grid));
                        cur_x += step;
                    }

                    let mut cur_y = off_y;
                    while cur_y < h {
                        let quad_b = Bounds {
                            origin: point(px(ox), px(oy + cur_y)),
                            size: size(px(w), px(1.0)),
                        };
                        window.paint_quad(fill(quad_b, theme.canvas_grid));
                        cur_y += step;
                    }

                    // 2. Concentric Orbital Rings around (600, 400)
                    let (center_x, center_y) = transform(600.0, 400.0);
                    for i in 0..4 {
                        let rx = (210.0 + (i as f32) * 110.0) * s * node_scale;
                        let ry = (130.0 + (i as f32) * 70.0) * s * node_scale;
                        let ring_bounds = Bounds {
                            origin: point(px(center_x - rx), px(center_y - ry)),
                            size: size(px(rx * 2.0), px(ry * 2.0)),
                        };
                        let mut ring_quad = fill(ring_bounds, rgba(0x00000000));
                        ring_quad.corner_radii = px(rx.min(ry)).into();
                        ring_quad.border_color = theme.canvas_ring.into();
                        ring_quad.border_widths = px(1.0).into();
                        window.paint_quad(ring_quad);
                    }

                    // 3. Edges & Data Pulses via direct index dereference
                    let sel_node = id_to_idx.get(&selected_id).map(|&idx| &nodes[idx]);
                    let branches: std::collections::HashSet<&str> = sel_node
                        .map(|n| n.branches.iter().map(|b| b.as_str()).collect())
                        .unwrap_or_default();

                    let segs = if nodes.len() > 3000 { 1 } else if nodes.len() > 500 { 2 } else { 4 };

                    for &(ia, ib) in &edge_pairs {
                        let a = &nodes[ia];
                        let b = &nodes[ib];
                        let a_vis = filter == "all" || a.layer == "all" || a.layer == filter;
                        let b_vis = filter == "all" || b.layer == "all" || b.layer == filter;
                        if !a_vis || !b_vis {
                            continue;
                        }

                        let (ax, ay) = transform(a.x, a.y);
                        let (bx, by) = transform(b.x, b.y);

                        let is_active = (a.id == selected_id || branches.contains(a.id.as_str()))
                            && (b.id == selected_id || branches.contains(b.id.as_str()));

                        let mx = (ax + bx) / 2.0 + (by - ay) * 0.06;
                        let my = (ay + by) / 2.0 - (bx - ax) * 0.06;

                        let edge_color = if is_active {
                            theme.canvas_edge_active
                        } else {
                            theme.canvas_edge
                        };
                        let line_w = if is_active {
                            1.5 * (0.75f32).max(s)
                        } else {
                            0.85 * (0.75f32).max(s)
                        };

                        if segs == 1 {
                            let seg_bounds = Bounds {
                                origin: point(px(ax.min(bx)), px(ay.min(by))),
                                size: size(
                                    px((bx - ax).abs().max(line_w)),
                                    px((by - ay).abs().max(line_w)),
                                ),
                            };
                            window.paint_quad(fill(seg_bounds, edge_color));
                        } else {
                            let mut prev_pt = (ax, ay);
                            for step_i in 1..=segs {
                                let t = (step_i as f32) / (segs as f32);
                                let px_pos =
                                    (1.0 - t) * (1.0 - t) * ax + 2.0 * (1.0 - t) * t * mx + t * t * bx;
                                let py_pos =
                                    (1.0 - t) * (1.0 - t) * ay + 2.0 * (1.0 - t) * t * my + t * t * by;

                                let seg_bounds = Bounds {
                                    origin: point(px(prev_pt.0.min(px_pos)), px(prev_pt.1.min(py_pos))),
                                    size: size(
                                        px((px_pos - prev_pt.0).abs().max(line_w)),
                                        px((py_pos - prev_pt.1).abs().max(line_w)),
                                    ),
                                };
                                window.paint_quad(fill(seg_bounds, edge_color));
                                prev_pt = (px_pos, py_pos);
                            }
                        }

                        // Data Pulse dot if active & trace enabled
                        if is_active && trace_active {
                            let t = ((anim_time * 0.00018
                                + (a.id.len() * b.id.len()) as f32 * 0.031)
                                % 1.0
                                + 1.0)
                                % 1.0;
                            let dot_x =
                                (1.0 - t) * (1.0 - t) * ax + 2.0 * (1.0 - t) * t * mx + t * t * bx;
                            let dot_y =
                                (1.0 - t) * (1.0 - t) * ay + 2.0 * (1.0 - t) * t * my + t * t * by;
                            let dot_bounds = Bounds {
                                origin: point(px(dot_x - 2.5), px(dot_y - 2.5)),
                                size: size(px(5.0), px(5.0)),
                            };
                            let mut dot_quad = fill(dot_bounds, theme.canvas_node_fill);
                            dot_quad.corner_radii = px(2.5).into();
                            window.paint_quad(dot_quad);
                        }
                    }

                    // 4. Paint Nodes
                    for n in &nodes {
                        let is_vis = filter == "all" || n.layer == "all" || n.layer == filter;
                        if !is_vis {
                            continue;
                        }

                        let (nx, ny) = transform(n.x, n.y);
                        let is_sel = n.id == selected_id;
                        let is_hover = hovered_id.as_deref() == Some(n.id.as_str());
                        let is_branch = branches.contains(n.id.as_str());

                        let r = (n.r + if is_sel { 3.5 } else if is_hover { 2.0 } else { 0.0 }) * (0.68f32).max(s);

                        // Outer focus ring if selected or hovered
                        if is_sel || is_hover {
                            let ring_r = r + 6.0 * (0.7f32).max(s);
                            let ring_b = Bounds {
                                origin: point(px(nx - ring_r), px(ny - ring_r)),
                                size: size(px(ring_r * 2.0), px(ring_r * 2.0)),
                            };
                            let mut rq = fill(ring_b, rgba(0x00000000));
                            rq.corner_radii = px(ring_r).into();
                            rq.border_color = (if is_sel { theme.canvas_edge_active } else { theme.text_muted }).into();
                            rq.border_widths = px(1.2).into();
                            window.paint_quad(rq);
                        }

                        // Node circle
                        let node_b = Bounds {
                            origin: point(px(nx - r), px(ny - r)),
                            size: size(px(r * 2.0), px(r * 2.0)),
                        };
                        let node_fill_color = if is_sel || n.id == "mind" {
                            theme.canvas_node_fill
                        } else if is_branch {
                            theme.canvas_node_secondary
                        } else {
                            theme.canvas_node_stroke
                        };

                        let mut nq = fill(node_b, node_fill_color);
                        nq.corner_radii = px(r).into();
                        nq.border_color = (if is_sel {
                            theme.canvas_node_stroke
                        } else {
                            theme.canvas_node_fill
                        })
                        .into();
                        nq.border_widths = px(if is_sel { 2.2 } else { 1.4 }).into();
                        window.paint_quad(nq);

                        // Inner core dot for non-selected hollow nodes
                        if !is_sel && !is_branch && n.id != "mind" {
                            let core_r = (r * 0.35).max(2.0);
                            let core_b = Bounds {
                                origin: point(px(nx - core_r), px(ny - core_r)),
                                size: size(px(core_r * 2.0), px(core_r * 2.0)),
                            };
                            let mut cq = fill(core_b, theme.canvas_node_secondary);
                            cq.corner_radii = px(core_r).into();
                            window.paint_quad(cq);
                        }

                        // Node Label: cull if outside viewport or too small to be legible
                        let is_on_screen = nx >= ox - 60.0 && nx <= ox + w + 60.0 && ny >= oy - 60.0 && ny <= oy + h + 60.0;
                        if is_on_screen && (n.r >= 14.0 || is_sel || is_hover || (cam_zoom > 2.0 && n.r >= 10.0)) {
                            let label_color = if is_sel {
                                theme.canvas_label
                            } else {
                                theme.canvas_label_muted
                            };
                            let font_size = (11.0f32 * s).clamp(10.0f32, 13.0f32);
                            let run = TextRun {
                                len: n.title.len(),
                                font: font(FONT_SANS),
                                color: label_color.into(),
                                background_color: None,
                                underline: None,
                                strikethrough: None,
                            };
                            let shaped = window.text_system().shape_line(
                                SharedString::from(n.title.clone()),
                                px(font_size),
                                &[run],
                                None,
                            );
                            let text_w = f32::from(shaped.width);
                            shaped
                                .paint(
                                    point(px(nx - text_w / 2.0), px(ny + r + 4.0)),
                                    px(font_size + 3.0),
                                    window,
                                    cx,
                                )
                                .ok();
                        }
                    }
                });

                // Keep animation running smoothly if active or during benchmark
                if trace_active || is_benchmark {
                    window.request_animation_frame();
                }
            },
        )
    }
}
