use crate::models::Page;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::*;

impl MindPlaceApp {
    pub fn render_settings_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let is_dark = theme.mode == ThemeMode::Dark;
        let is_compact = self.state.density == DensityMode::Compact;
        let tech_mode = self.state.tech_mode;

        div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(1340.0))
            .mx_auto()
            .gap(px(20.0))
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
                                    .child("WORKSPACE & ERGONOMIC PREFERENCES"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("Settings"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child("Customize monochrome visual contrast, information density, and technical provenance telemetry."),
                            ),
                    ),
            )
            // Two-column grid
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    // Panel 1: Interface & Visual Ergonomics
                    .child(
                        div()
                            .flex_1()
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).child("Interface & Visual Ergonomics"))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mb(px(16.0)).child("Strictly neutral white-to-black design system for zero chromatic distraction."))
                            // Row 1: Theme Switch
                            .child(
                                div()
                                    .py(px(14.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Monochrome Color Theme"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Switch between Crisp Daylight White and High-Contrast Obsidian Black.")),
                                    )
                                    .child(
                                        div()
                                            .id("settings-theme-toggle-btn")
                                            .h(px(32.0))
                                            .px(px(12.0))
                                            .rounded(RADIUS_SM)
                                            .border_1()
                                            .border_color(theme.border_strong)
                                            .bg(theme.bg_surface)
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.bg_hover))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.toggle_theme();
                                                cx.notify();
                                            }))
                                            .child(if is_dark {
                                                "Obsidian Dark (#09090B)"
                                            } else {
                                                "Pure Light (#F4F4F5 / #FFF)"
                                            }),
                                    ),
                            )
                            // Row 2: Tech Mode Switch
                            .child(
                                div()
                                    .py(px(14.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Technical Provenance Mode (DEV / ⌘I)"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Display SHA-256 hashes, SQLCipher states, and BM25 rank scores inline.")),
                                    )
                                    .child(
                                        div()
                                            .id("settings-tech-switch-btn")
                                            .cursor_pointer()
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.toggle_tech_mode();
                                                cx.notify();
                                            }))
                                            .child(self.render_switch(tech_mode)),
                                    ),
                            )
                            // Row 3: Density Switch
                            .child(
                                div()
                                    .py(px(14.0))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Compact Table & List Density"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Tighten vertical padding to view more records per screen.")),
                                    )
                                    .child(
                                        div()
                                            .id("settings-density-switch-btn")
                                            .cursor_pointer()
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.toggle_density();
                                                cx.notify();
                                            }))
                                            .child(self.render_switch(is_compact)),
                                    ),
                            ),
                    )
                    // Panel 2: Storage & Authority Controls
                    .child(
                        div()
                            .w(px(460.0))
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).child("Storage & Authority Controls"))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mb(px(16.0)).child("Direct shortcuts to local database and MCP governance."))
                            .child(
                                div()
                                    .py(px(14.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Local-First Source of Truth"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("SQLite + SQLCipher vault resides strictly on your machine.")),
                                    )
                                    .child(self.render_status_badge("VERIFIED", true)),
                            )
                            .child(
                                div()
                                    .py(px(14.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Provider Capability Matrix"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Audit and revoke per-provider read/write scopes.")),
                                    )
                                    .child(
                                        div()
                                            .id("settings-goto-providers-btn")
                                            .h(px(28.0))
                                            .px(px(10.0))
                                            .rounded(RADIUS_XS)
                                            .border_1()
                                            .border_color(theme.border_strong)
                                            .bg(theme.bg_surface)
                                            .text_size(px(11.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.bg_hover))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.navigate(Page::Providers);
                                                cx.notify();
                                            }))
                                            .child("Configure Providers →"),
                                    ),
                            )
                            .child(
                                div()
                                    .py(px(14.0))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Vault Lock & Recovery"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Test manual vault lock screen or inspect encryption profiles.")),
                                    )
                                    .child(
                                        div()
                                            .id("settings-goto-files-btn")
                                            .h(px(28.0))
                                            .px(px(10.0))
                                            .rounded(RADIUS_XS)
                                            .border_1()
                                            .border_color(theme.border_strong)
                                            .bg(theme.bg_surface)
                                            .text_size(px(11.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.bg_hover))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.navigate(Page::Files);
                                                cx.notify();
                                            }))
                                            .child("Manage Vault →"),
                                    ),
                            ),
                    ),
            )
    }
}
