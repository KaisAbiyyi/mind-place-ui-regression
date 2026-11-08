use crate::models::{DialogModal, MemoryViewTab, Page};
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    pub fn render_memory_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let tech_mode = self.state.tech_mode;
        let query = self.state.memory_search_query.to_lowercase();
        let view_tab = self.state.memory_view_tab;

        let filtered_collections: Vec<_> = self
            .state
            .collections
            .iter()
            .filter(|c| {
                query.is_empty()
                    || c.name.to_lowercase().contains(&query)
                    || c.desc.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        let filtered_memories: Vec<_> = self
            .state
            .memories
            .iter()
            .filter(|m| {
                query.is_empty()
                    || m.title.to_lowercase().contains(&query)
                    || m.content.to_lowercase().contains(&query)
                    || m.collection.to_lowercase().contains(&query)
                    || m.mem_type.to_lowercase().contains(&query)
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
                                    .child("STATE-CORE · CURATED MEMORY & RANKING"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("Curated Memory"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child("Distilled, high-signal knowledge units extracted from conversations and local documents with strict source provenance."),
                            ),
                    )
                    .child(
                        div()
                            .id("new-collection-btn")
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
                                    kicker: "STATE-CORE COLLECTION".into(),
                                    title: "Create Curated Memory Collection".into(),
                                    body_html: "Create a new topic boundary to group distilled insights, authority rankings, and document extractions.".into(),
                                });
                                cx.notify();
                            }))
                            .child("＋ New Collection"),
                    ),
            )
            // Toolbar
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
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
                            .child("Search 342 curated memories, insights, rules, and collections…"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(2.0))
                            .p(px(3.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_subtle)
                            .child(
                                div()
                                    .id("mem-tab-cols")
                                    .h(px(28.0))
                                    .px(px(12.0))
                                    .rounded(px(5.0))
                                    .text_size(px(12.0))
                                    .font_weight(if view_tab == MemoryViewTab::Collections { FontWeight::SEMIBOLD } else { FontWeight::MEDIUM })
                                    .bg(if view_tab == MemoryViewTab::Collections { theme.bg_surface } else { theme.bg_subtle })
                                    .text_color(if view_tab == MemoryViewTab::Collections { theme.text_primary } else { theme.text_secondary })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.memory_view_tab = MemoryViewTab::Collections;
                                        cx.notify();
                                    }))
                                    .child("Collections Grid (6)"),
                            )
                            .child(
                                div()
                                    .id("mem-tab-all")
                                    .h(px(28.0))
                                    .px(px(12.0))
                                    .rounded(px(5.0))
                                    .text_size(px(12.0))
                                    .font_weight(if view_tab == MemoryViewTab::All { FontWeight::SEMIBOLD } else { FontWeight::MEDIUM })
                                    .bg(if view_tab == MemoryViewTab::All { theme.bg_surface } else { theme.bg_subtle })
                                    .text_color(if view_tab == MemoryViewTab::All { theme.text_primary } else { theme.text_secondary })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.memory_view_tab = MemoryViewTab::All;
                                        cx.notify();
                                    }))
                                    .child("All Curated Memories"),
                            ),
                    ),
            )
            // Collections Grid (when Collections view selected)
            .when(view_tab == MemoryViewTab::Collections, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(16.0))
                        .children(filtered_collections.into_iter().map(|col| {
                            let col_name = col.name.to_string();
                            div()
                                .id(SharedString::from(format!("mem-grid-col-{}", col.name)))
                                .w(px(430.0))
                                .p(px(20.0))
                                .rounded(RADIUS_MD)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .bg(theme.bg_surface)
                                .flex()
                                .flex_col()
                                .justify_between()
                                .cursor_pointer()
                                .hover(|s| s.border_color(theme.border_strong))
                                .on_click(cx.listener(move |this, _event, _window, cx| {
                                    this.state.selected_collection_name = col_name.clone();
                                    this.state.navigate(Page::CollectionDetail);
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .w(px(32.0))
                                                .h(px(32.0))
                                                .rounded(RADIUS_SM)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .bg(theme.bg_subtle)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_size(px(13.0))
                                                .text_color(theme.text_primary)
                                                .child(match col.name {
                                                    "Engineering" => "⌘",
                                                    "Research" => "⌕",
                                                    "Learning" => "◇",
                                                    "Life & goals" => "○",
                                                    "Product ideas" => "▧",
                                                    _ => "＋",
                                                }),
                                        )
                                        .child(
                                            div()
                                                .font_family(FONT_MONO)
                                                .text_size(px(11.0))
                                                .px(px(6.0))
                                                .py(px(2.0))
                                                .rounded(px(4.0))
                                                .bg(theme.bg_subtle)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .text_color(theme.text_secondary)
                                                .child(format!("{} memories", col.count)),
                                        ),
                                )
                                .child(
                                    div()
                                        .mt(px(14.0))
                                        .mb(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child(col.name),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.5))
                                                .text_color(theme.text_secondary)
                                                .mt(px(4.0))
                                                .child(col.desc),
                                        ),
                                )
                                .child(
                                    div()
                                        .child(
                                            div()
                                                .h(px(4.0))
                                                .w_full()
                                                .rounded_full()
                                                .bg(theme.bg_muted)
                                                .overflow_hidden()
                                                .child(
                                                    div()
                                                        .h_full()
                                                        .w(px((col.pct as f32) * 3.8))
                                                        .rounded_full()
                                                        .bg(theme.text_primary),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .mt(px(8.0))
                                                .text_size(px(11.5))
                                                .text_color(theme.text_muted)
                                                .child(format!("Authority density: {}%", col.pct))
                                                .child("Inspect →"),
                                        ),
                                )
                        })),
                )
            })
            // High-Authority Curated Memories Card
            .child(
                div()
                    .w_full()
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded(RADIUS_MD)
                    .flex()
                    .flex_col()
                    .child(
                        // Card Header
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(16.0))
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child("High-Authority Curated Memories"),
                            )
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .text_color(theme.text_secondary)
                                    .child("BM25 + Semantic Ranked"),
                            ),
                    )
                    .child(
                        // Memories List
                        div()
                            .p(px(8.0))
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .children(filtered_memories.into_iter().take(10).map(|m| {
                                let id = m.id.clone();
                                let id_clone = m.id.clone();
                                div()
                                    .id(SharedString::from(format!("mem-row-{}", m.id)))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .p(px(11.0))
                                    .rounded(RADIUS_SM)
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .child(
                                        div()
                                            .id(SharedString::from(format!("mem-cell-{}", id)))
                                            .flex_1()
                                            .flex()
                                            .items_center()
                                            .gap(px(12.0))
                                            .cursor_pointer()
                                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                                this.state.selected_memory_id = id.clone();
                                                this.state.navigate(Page::MemoryDetail);
                                                cx.notify();
                                            }))
                                            .child(
                                                div()
                                                    .w(px(32.0))
                                                    .h(px(32.0))
                                                    .rounded(RADIUS_SM)
                                                    .border_1()
                                                    .border_color(theme.border_subtle)
                                                    .bg(theme.bg_subtle)
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(13.0))
                                                    .text_color(theme.text_primary)
                                                    .child(match m.collection {
                                                        "Engineering" => "⌘",
                                                        "Research" => "⌕",
                                                        "Learning" => "◇",
                                                        "Life & goals" => "○",
                                                        "Product ideas" => "▧",
                                                        _ => "＋",
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .text_size(px(13.5))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(theme.text_primary)
                                                            .child(m.title),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(8.0))
                                                            .text_size(px(12.0))
                                                            .text_color(theme.text_muted)
                                                            .child(m.collection)
                                                            .child("·")
                                                            .child(m.mem_type)
                                                            .child("·")
                                                            .child(format!("Source: {}", m.source))
                                                            .when(tech_mode, |this| {
                                                                this.child("·").child(
                                                                    div()
                                                                        .font_family(FONT_MONO)
                                                                        .child(format!("rank:{}", m.score)),
                                                                )
                                                            }),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(6.0))
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!("peek-btn-{}", m.id)))
                                                    .h(px(30.0))
                                                    .px(px(10.0))
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
                                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                                        this.state.quick_drawer_memory_id = Some(id_clone.clone());
                                                        cx.notify();
                                                    }))
                                                    .child("Quick Peek"),
                                            )
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!("full-view-btn-{}", m.id)))
                                                    .h(px(30.0))
                                                    .px(px(10.0))
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
                                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                                        this.state.selected_memory_id = m.id.clone();
                                                        this.state.navigate(Page::MemoryDetail);
                                                        cx.notify();
                                                    }))
                                                    .child("Full View →"),
                                            ),
                                    )
                            })),
                    ),
            )
    }
}
