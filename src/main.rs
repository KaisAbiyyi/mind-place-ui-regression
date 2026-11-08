mod app_state;
mod assets;
mod components;
mod hive_data;
mod models;
mod screens;
#[cfg(test)]
mod tests;
mod theme;

use app_state::AppState;
use gpui::*;
use models::Page;

pub struct MindPlaceApp {
    pub state: AppState,
    pub focus_handle: FocusHandle,
}

impl MindPlaceApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self::new_with_state(AppState::new(), cx)
    }

    pub fn new_with_state(state: AppState, cx: &mut Context<Self>) -> Self {
        Self {
            state,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for MindPlaceApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let active_page = self.state.active_page;

        // Ensure root has focus so key combinations work immediately
        window.focus(&self.focus_handle);

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                let ctrl = event.keystroke.modifiers.control || event.keystroke.modifiers.platform;
                let key = event.keystroke.key.to_lowercase();

                if key == "escape" {
                    this.state.cmd_palette_open = false;
                    this.state.quick_drawer_memory_id = None;
                    this.state.quick_drawer_folder_id = None;
                    this.state.add_folder_modal_open = false;
                    this.state.dialog_modal = None;
                    cx.notify();
                } else if ctrl && key == "k" {
                    this.state.cmd_palette_open = !this.state.cmd_palette_open;
                    this.state.cmd_query.clear();
                    cx.notify();
                } else if ctrl && key == "b" {
                    this.state.toggle_sidebar();
                    cx.notify();
                } else if ctrl && key == "d" {
                    this.state.toggle_theme();
                    cx.notify();
                } else if ctrl && key == "i" {
                    this.state.toggle_tech_mode();
                    cx.notify();
                }
            }))
            .size_full()
            .relative()
            .flex()
            .bg(theme.bg_app)
            .text_color(theme.text_primary)
            .overflow_hidden()
            // Sidebar
            .child(self.render_sidebar(cx))
            // Main workspace
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .h_full()
                    .min_w(px(0.0))
                    .overflow_hidden()
                    // Topbar
                    .child(self.render_topbar(cx))
                    // Scrollable content area
                    .child(
                        div()
                            .id("main-content-scroll")
                            .flex_1()
                            .overflow_y_scroll()
                            .px(px(32.0))
                            .py(px(28.0))
                            .child(match active_page {
                                Page::Home => self.render_overview_screen(cx).into_any_element(),
                                Page::Conversations => {
                                    self.render_conversations_screen(cx).into_any_element()
                                }
                                Page::Memory => self.render_memory_screen(cx).into_any_element(),
                                Page::Hive => self.render_hive_screen(cx).into_any_element(),
                                Page::Files => self.render_files_screen(cx).into_any_element(),
                                Page::Providers => {
                                    self.render_providers_screen(cx).into_any_element()
                                }
                                Page::Setup => self.render_setup_screen(cx).into_any_element(),
                                Page::Settings => {
                                    self.render_settings_screen(cx).into_any_element()
                                }
                                Page::ConversationDetail => self
                                    .render_conversation_detail_screen(cx)
                                    .into_any_element(),
                                Page::CollectionDetail => {
                                    self.render_collection_detail_screen(cx).into_any_element()
                                }
                                Page::MemoryDetail => {
                                    self.render_memory_detail_screen(cx).into_any_element()
                                }
                            }),
                    ),
            )
            // Global overlays (Command Palette, Drawer, Add Folder Modal, Dialog Modal, Lock Screen, Toast)
            .children(self.render_overlays(cx))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut bench_dataset: Option<String> = None;
    let mut bench_scenario = "stress".to_string();
    let mut bench_duration = 10.0f64;
    let mut bench_output: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--benchmark" => {
                if i + 1 < args.len() {
                    bench_dataset = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--scenario" => {
                if i + 1 < args.len() {
                    bench_scenario = args[i + 1].clone();
                    i += 1;
                }
            }
            "--duration" => {
                if i + 1 < args.len() {
                    if let Ok(d) = args[i + 1].parse::<f64>() {
                        bench_duration = d;
                    }
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    bench_output = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    Application::new()
        .with_assets(assets::EmbeddedAssets)
        .run(move |cx: &mut App| {
            let mut state = AppState::new();
            if let Some(ref ds_path) = bench_dataset {
                let load_t0 = std::time::Instant::now();
                match std::fs::read_to_string(ds_path) {
                    Ok(content) => {
                        let load_ms = load_t0.elapsed().as_secs_f64() * 1000.0;
                        if let Err(e) = state.load_hive_dataset(&content) {
                            eprintln!("Failed to parse dataset {}: {}", ds_path, e);
                        } else {
                            state.active_page = Page::Hive;
                            let now = std::time::Instant::now();
                            state.hive_benchmark = Some(app_state::BenchmarkRunner {
                                scenario: bench_scenario.clone(),
                                duration_sec: bench_duration,
                                start_time: now,
                                last_frame_time: now,
                                frame_times: Vec::new(),
                                dataset_load_time_ms: load_ms,
                                output_file: bench_output.clone(),
                            });
                        }
                    }
                    Err(e) => eprintln!("Failed to read dataset file {}: {}", ds_path, e),
                }
            }

            let bounds = Bounds::centered(None, size(px(1440.0), px(900.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(
                            "Mind Place — Personal Knowledge OS & Shared-Brain Archive".into(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| MindPlaceApp::new_with_state(state, cx)),
            )
            .unwrap();
        });
}
