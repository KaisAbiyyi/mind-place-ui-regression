use crate::models::DialogModal;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::*;

impl MindPlaceApp {
    pub fn render_topbar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let crumb = self.state.active_page.name();
        let tech_mode = self.state.tech_mode;

        div()
            .flex()
            .items_center()
            .h(px(58.0))
            .min_h(px(58.0))
            .bg(theme.bg_surface)
            .border_b_1()
            .border_color(theme.border_subtle)
            .px(px(28.0))
            .gap(px(16.0))
            .child(
                // Breadcrumbs
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .text_size(px(13.0))
                    .text_color(theme.text_muted)
                    .child("Mind Place")
                    .child("/")
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_primary)
                            .child(crumb),
                    ),
            )
            .child(
                // Command palette trigger
                div()
                    .id("command-trigger-btn")
                    .flex_1()
                    .max_w(px(440.0))
                    .h(px(36.0))
                    .ml_auto()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .px(px(12.0))
                    .rounded(RADIUS_SM)
                    .bg(theme.bg_subtle)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .text_color(theme.text_muted)
                    .text_size(px(12.5))
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.bg_surface).border_color(theme.border_strong).text_color(theme.text_primary))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.state.cmd_palette_open = true;
                        this.state.cmd_query.clear();
                        cx.notify();
                    }))
                    .child(
                        svg()
                            .path("icons/search.svg")
                            .w(px(15.0))
                            .h(px(15.0))
                            .text_color(theme.text_muted),
                    )
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child("Search conversations, memories, Hive nodes, or run command…"),
                    )
                    .child(
                        div()
                            .font_family(FONT_MONO)
                            .text_size(px(11.0))
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(theme.border_strong)
                            .bg(theme.bg_surface)
                            .text_color(theme.text_secondary)
                            .child("⌘K"),
                    ),
            )
            .child(
                // Top actions
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        // DEV / PROVENANCE toggle
                        div()
                            .id("tech-mode-pill-btn")
                            .h(px(32.0))
                            .px(px(10.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(if tech_mode { theme.bg_inverse } else { theme.border_subtle })
                            .bg(if tech_mode { theme.bg_inverse } else { theme.bg_surface })
                            .text_color(if tech_mode { theme.text_inverse } else { theme.text_secondary })
                            .font_family(FONT_MONO)
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .flex()
                            .items_center()
                            .cursor_pointer()
                            .hover(move |s| {
                                if !tech_mode {
                                    s.border_color(theme.border_strong).text_color(theme.text_primary)
                                } else {
                                    s
                                }
                            })
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.state.toggle_tech_mode();
                                cx.notify();
                            }))
                            .child("DEV / PROVENANCE"),
                    )
                    .child(
                        // Theme toggle
                        div()
                            .id("theme-toggle-btn")
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .text_color(theme.text_secondary)
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover).border_color(theme.border_strong).text_color(theme.text_primary))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.state.toggle_theme();
                                cx.notify();
                            }))
                            .child(
                                svg()
                                    .path("icons/theme.svg")
                                    .w(px(15.0))
                                    .h(px(15.0))
                                    .text_color(theme.text_secondary),
                            ),
                    )
                    .child(
                        // Ask AI button
                        div()
                            .id("ask-ai-btn")
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .text_color(theme.text_secondary)
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover).border_color(theme.border_strong).text_color(theme.text_primary))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.state.dialog_modal = Some(DialogModal {
                                    kicker: "CONTEXT ENGINE RETRIEVAL".into(),
                                    title: "Query Your Shared-Brain Archive".into(),
                                    body_html: "Ask a question across all 128 conversations and 342 curated memories. Answers synthesize only verified local context and preserve exact source links.\n\nSynthesized Answer (from Engineering · 0.98 rank):\nBOMC Exact must preserve canonical geometry, material assignment, and accepted visual output while reducing package size relative to the source SKP.".into(),
                                });
                                cx.notify();
                            }))
                            .child(
                                svg()
                                    .path("icons/ask_ai.svg")
                                    .w(px(15.0))
                                    .h(px(15.0))
                                    .text_color(theme.text_secondary),
                            ),
                    ),
            )
    }
}
