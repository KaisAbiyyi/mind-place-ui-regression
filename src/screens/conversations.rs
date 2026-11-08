use crate::models::{ConvTabFilter, Page};
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    pub fn render_conversations_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let tech_mode = self.state.tech_mode;
        let density = self.state.density;
        let query = self.state.conv_search_query.to_lowercase();
        let provider_filter = self.state.conv_provider_filter.clone();
        let tab_filter = self.state.conv_tab_filter;

        let filtered_convs: Vec<_> = self
            .state
            .conversations
            .iter()
            .filter(|c| {
                let matches_q = query.is_empty()
                    || c.title.to_lowercase().contains(&query)
                    || c.collection.to_lowercase().contains(&query)
                    || c.provider.to_lowercase().contains(&query);

                let matches_prov = provider_filter == "all" || c.provider == provider_filter;

                let is_starred = self.state.starred_conversations.contains(c.title);
                let matches_tab = match tab_filter {
                    ConvTabFilter::All => true,
                    ConvTabFilter::Starred => is_starred,
                    ConvTabFilter::Archived => c.timestamp.contains("Oct"),
                };

                matches_q && matches_prov && matches_tab
            })
            .cloned()
            .collect();

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
                                    .child("CAPTURE HOST & ARCHIVE-CORE"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("Conversations Archive"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child("Search, inspect provenance, and revisit full multi-provider transcripts captured into your local vault."),
                            ),
                    )
                    .child(
                        div()
                            .id("sim-capture-btn")
                            .h(px(36.0))
                            .px(px(14.0))
                            .rounded(RADIUS_SM)
                            .bg(theme.bg_inverse)
                            .text_color(theme.text_inverse)
                            .text_size(px(12.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.state.set_toast("Simulated browser capture: +1 conversation synced from capture-host");
                                cx.notify();
                            }))
                            .child("＋ Simulate Capture Ingestion"),
                    ),
            )
            // Toolbar
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        // Search bar
                        div()
                            .flex_1()
                            .h(px(36.0))
                            .px(px(12.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_strong)
                            .bg(theme.bg_surface)
                            .flex()
                            .items_center()
                            .text_size(px(13.0))
                            .text_color(theme.text_primary)
                            .child(if self.state.conv_search_query.is_empty() {
                                "Filter conversations by title, collection, or provider…".to_string()
                            } else {
                                self.state.conv_search_query.clone()
                            }),
                    )
                    .child(
                        // Provider Filter Dropdown / Pills
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .p(px(3.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_subtle)
                            .child(self.render_filter_option("All Providers", "all", cx))
                            .child(self.render_filter_option("ChatGPT", "ChatGPT", cx))
                            .child(self.render_filter_option("Claude", "Claude", cx))
                            .child(self.render_filter_option("Gemini", "Gemini", cx)),
                    )
                    .child(
                        // Tabs
                        div()
                            .flex()
                            .items_center()
                            .gap(px(2.0))
                            .p(px(3.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_subtle)
                            .child(self.render_conv_tab_button("All (8)", ConvTabFilter::All, cx))
                            .child(self.render_conv_tab_button("Starred", ConvTabFilter::Starred, cx))
                            .child(self.render_conv_tab_button("Cold Archive", ConvTabFilter::Archived, cx)),
                    ),
            )
            // Table Card
            .child(
                div()
                    .w_full()
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded(RADIUS_MD)
                    .overflow_hidden()
                    .child(
                        // Table Header
                        div()
                            .flex()
                            .items_center()
                            .bg(theme.bg_subtle)
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .px(px(16.0))
                            .py(px(12.0))
                            .font_family(FONT_MONO)
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_muted)
                            .child(div().w(px(380.0)).child("CONVERSATION TITLE"))
                            .child(div().w(px(120.0)).child("PROVIDER"))
                            .child(div().w(px(140.0)).child("COLLECTION"))
                            .when(tech_mode, |this| {
                                this.child(div().w(px(220.0)).child("PROVENANCE SHA / MSGS"))
                            })
                            .child(div().flex_1().child("LAST ACTIVITY"))
                            .child(div().w(px(100.0)).text_align(TextAlign::Right).child("INSPECT")),
                    )
                    .child(
                        // Table Rows
                        div()
                            .flex()
                            .flex_col()
                            .children(if filtered_convs.is_empty() {
                                vec![div()
                                    .p(px(32.0))
                                    .text_align(TextAlign::Center)
                                    .text_color(theme.text_muted)
                                    .child("No conversations match the current filter criteria.")
                                    .into_any_element()]
                            } else {
                                filtered_convs.into_iter().enumerate().map(|(idx, c)| {
                                    let is_starred = self.state.starred_conversations.contains(c.title);
                                    let title = c.title.to_string();
                                    let pad_y = if density == DensityMode::Compact { px(9.0) } else { px(13.0) };

                                    div()
                                        .id(SharedString::from(format!("conv-row-{}", idx)))
                                        .flex()
                                        .items_center()
                                        .px(px(16.0))
                                        .py(pad_y)
                                        .border_b_1()
                                        .border_color(theme.border_subtle)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_click(cx.listener(move |this, _event, _window, cx| {
                                            this.state.selected_conversation_title = title.clone();
                                            this.state.navigate(Page::ConversationDetail);
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .w(px(380.0))
                                                .text_size(px(13.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.text_primary)
                                                .flex()
                                                .items_center()
                                                .gap(px(6.0))
                                                .child(if is_starred { "★" } else { "" })
                                                .child(c.title),
                                        )
                                        .child(
                                            div()
                                                .w(px(120.0))
                                                .child(
                                                    div()
                                                        .px(px(8.0))
                                                        .py(px(2.0))
                                                        .rounded(px(5.0))
                                                        .border_1()
                                                        .border_color(theme.border_subtle)
                                                        .bg(theme.bg_subtle)
                                                        .text_size(px(11.5))
                                                        .text_color(theme.text_secondary)
                                                        .child(c.provider),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .w(px(140.0))
                                                .text_size(px(13.0))
                                                .text_color(theme.text_secondary)
                                                .child(c.collection),
                                        )
                                        .when(tech_mode, |this| {
                                            this.child(
                                                div()
                                                    .w(px(220.0))
                                                    .font_family(FONT_MONO)
                                                    .text_size(px(11.5))
                                                    .text_color(theme.text_muted)
                                                    .child(format!("{} · {} msgs", c.sha, c.msgs)),
                                            )
                                        })
                                        .child(
                                            div()
                                                .flex_1()
                                                .font_family(FONT_MONO)
                                                .text_size(px(12.0))
                                                .text_color(theme.text_muted)
                                                .child(c.timestamp),
                                        )
                                        .child(
                                            div()
                                                .w(px(100.0))
                                                .flex()
                                                .justify_end()
                                                .child(
                                                    div()
                                                        .h(px(28.0))
                                                        .px(px(10.0))
                                                        .rounded(RADIUS_XS)
                                                        .border_1()
                                                        .border_color(theme.border_strong)
                                                        .bg(theme.bg_surface)
                                                        .text_size(px(12.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child("Reader →"),
                                                ),
                                        )
                                        .into_any_element()
                                }).collect()
                            }),
                    ),
            )
    }

    fn render_filter_option(
        &mut self,
        label: &'static str,
        val: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_active = self.state.conv_provider_filter == val;

        div()
            .id(SharedString::from(format!("prov-filt-{}", val)))
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(5.0))
            .text_size(px(12.0))
            .font_weight(if is_active {
                FontWeight::SEMIBOLD
            } else {
                FontWeight::MEDIUM
            })
            .bg(if is_active {
                theme.bg_surface
            } else {
                theme.bg_subtle
            })
            .text_color(if is_active {
                theme.text_primary
            } else {
                theme.text_secondary
            })
            .cursor_pointer()
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.state.conv_provider_filter = val.to_string();
                cx.notify();
            }))
            .child(label)
    }

    fn render_conv_tab_button(
        &mut self,
        label: &'static str,
        tab: ConvTabFilter,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_active = self.state.conv_tab_filter == tab;

        div()
            .id(SharedString::from(format!("conv-tab-{:?}", tab)))
            .h(px(28.0))
            .px(px(12.0))
            .rounded(px(5.0))
            .text_size(px(12.0))
            .font_weight(if is_active {
                FontWeight::SEMIBOLD
            } else {
                FontWeight::MEDIUM
            })
            .bg(if is_active {
                theme.bg_surface
            } else {
                theme.bg_subtle
            })
            .text_color(if is_active {
                theme.text_primary
            } else {
                theme.text_secondary
            })
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.state.conv_tab_filter = tab;
                cx.notify();
            }))
            .child(label)
    }
}
