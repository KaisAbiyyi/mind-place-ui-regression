use crate::models::Page;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    // ==========================================
    // 1. Conversation Detail Reader Screen
    // ==========================================
    pub fn render_conversation_detail_screen(
        &mut self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let title = self.state.selected_conversation_title.clone();
        let conv = self
            .state
            .conversations
            .iter()
            .find(|c| c.title == title)
            .cloned()
            .unwrap_or_else(|| self.state.conversations[0].clone());

        let is_starred = self.state.starred_conversations.contains(conv.title);
        let conv_title = conv.title.to_string();

        let messages = match conv.title {
            "BOMC Builder — export architecture" => vec![
                ("user", "The goal is to make BOMC Exact faster and smaller than the source SKP without losing fidelity. What should the acceptance gate measure?"),
                ("assistant", "Measure package size and export duration independently for each model, then compare canonical geometry and material assignments. A smaller package is not a pass if the Exact output changes."),
                ("user", "What about visual quality?"),
                ("assistant", "Keep visual review separate from structural equivalence. An independent architect should inspect representative outputs and record visible differences, artifacts, and practical acceptability."),
            ],
            "Research: semantic retrieval" => vec![
                ("user", "How should memory retrieval behave when a query matches several related conversations?"),
                ("assistant", "Return the strongest relevant memories with provenance. Keep the source conversation, provider, timestamp, and extraction path attached so the user can verify the result."),
                ("user", "Should related memories be merged automatically?"),
                ("assistant", "Only when the evidence supports a stable relationship. Keep ambiguous or isolated items visible and avoid forcing them into a cluster."),
            ],
            "Hive graph product direction" => vec![
                ("user", "I want the graph to feel like a real knowledge space, not a static mind map."),
                ("assistant", "Use dense local clusters, selective cross-links, and spatial exploration. Hover should reveal context; selecting a node should open its branch and source trail."),
                ("user", "What happens to memories that do not connect?"),
                ("assistant", "Keep them as isolated memories. Uncertainty should be visible rather than hidden by invented relationships."),
            ],
            _ => vec![
                ("user", "I want to understand the current direction and what is already established. What should we preserve, and what still needs work?"),
                ("assistant", "The important distinction is between what has been demonstrated and what is still a target. We should preserve the accepted behavior, make remaining requirements explicit, and avoid treating an unverified path as complete."),
                ("user", "Can you turn that into something actionable?"),
                ("assistant", "Start with the acceptance criteria, attach each one to evidence, and keep the source traceable. Any unresolved dependency should remain visible instead of being silently assumed complete."),
            ],
        };

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
                                    .child(format!(
                                        "CONVERSATION ARCHIVE / {} · {}",
                                        conv.provider.to_uppercase(),
                                        conv.sha
                                    )),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child(conv.title),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child(format!(
                                        "{} · Collection: {} · Captured {}",
                                        conv.provider, conv.collection, conv.timestamp
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("conv-back-btn")
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
                                        this.state.navigate(Page::Conversations);
                                        cx.notify();
                                    }))
                                    .child("← Back to Conversations"),
                            )
                            .child(
                                div()
                                    .id("copy-transcript-btn")
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
                                        this.state.set_toast("Full transcript copied to clipboard");
                                        cx.notify();
                                    }))
                                    .child("Copy Transcript"),
                            ),
                    ),
            )
            // Two-column layout: Transcript + Metadata Aside
            .child(
                div()
                    .flex()
                    .gap(px(18.0))
                    // Transcript Main Card
                    .child(
                        div()
                            .flex_1()
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .overflow_hidden()
                            .child(
                                div()
                                    .h(px(52.0))
                                    .px(px(20.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(10.0))
                                            .child(
                                                div()
                                                    .text_size(px(13.5))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Verified Transcript"),
                                            )
                                            .child(
                                                div()
                                                    .font_family(FONT_MONO)
                                                    .text_size(px(11.0))
                                                    .px(px(6.0))
                                                    .py(px(2.0))
                                                    .rounded(px(4.0))
                                                    .bg(theme.bg_subtle)
                                                    .child(format!("{} messages", messages.len())),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .text_color(theme.text_muted)
                                            .child("adapter: dom-snapshot-v3 · lossless"),
                                    ),
                            )
                            .child(div().p(px(24.0)).flex().flex_col().gap(px(20.0)).children(
                                messages.into_iter().enumerate().map(|(idx, (role, text))| {
                                    let is_user = role == "user";
                                    div()
                                        .pb(px(18.0))
                                        .border_b_1()
                                        .border_color(theme.border_subtle)
                                        .flex()
                                        .flex_col()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(10.0))
                                                .child(
                                                    div()
                                                        .w(px(26.0))
                                                        .h(px(26.0))
                                                        .rounded(px(6.0))
                                                        .border_1()
                                                        .border_color(theme.border_strong)
                                                        .bg(theme.bg_subtle)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .font_family(FONT_MONO)
                                                        .text_size(px(11.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(if is_user {
                                                            "KA"
                                                        } else {
                                                            &conv.provider[..1]
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child(if is_user {
                                                            "Kais Abiyyi (Owner)"
                                                        } else {
                                                            conv.provider
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .ml_auto()
                                                        .font_family(FONT_MONO)
                                                        .text_size(px(11.0))
                                                        .text_color(theme.text_muted)
                                                        .child(format!(
                                                            "msg_{} · {}",
                                                            idx + 1,
                                                            if idx == 0 {
                                                                "Initial Prompt"
                                                            } else {
                                                                "Turn"
                                                            }
                                                        )),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .pl(px(36.0))
                                                .text_size(px(14.0))
                                                .line_height(px(24.0))
                                                .text_color(theme.text_primary)
                                                .child(text),
                                        )
                                }),
                            )),
                    )
                    // Aside Metadata Card
                    .child(
                        div()
                            .w(px(310.0))
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(12.0))
                                    .child("PROVENANCE & METADATA"),
                            )
                            .child(self.render_detail_meta_row("AI Provider", conv.provider, false))
                            .child(self.render_detail_meta_row(
                                "Collection Scope",
                                conv.collection,
                                false,
                            ))
                            .child(self.render_detail_meta_row(
                                "Captured Timestamp",
                                conv.timestamp,
                                false,
                            ))
                            .child(self.render_detail_meta_row("Content Hash", conv.sha, true))
                            .child(self.render_detail_meta_row(
                                "Storage State",
                                "Encrypted in archive-core",
                                false,
                            ))
                            // Divider
                            .child(div().h(px(1.0)).bg(theme.border_subtle).my(px(16.0)))
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(12.0))
                                    .child("EXTRACTED CURATED MEMORIES"),
                            )
                            .child(
                                div().flex().flex_col().gap(px(8.0)).children(
                                    self.state
                                        .memories
                                        .iter()
                                        .filter(|m| {
                                            m.source == conv.title
                                                || m.collection == conv.collection
                                        })
                                        .take(3)
                                        .cloned()
                                        .map(|m| {
                                            let mid = m.id.clone();
                                            div()
                                                .id(SharedString::from(format!(
                                                    "conv-rel-mem-{}",
                                                    m.id
                                                )))
                                                .p(px(10.0))
                                                .rounded(RADIUS_SM)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .bg(theme.bg_subtle)
                                                .cursor_pointer()
                                                .hover(|s| s.border_color(theme.border_focus))
                                                .on_click(cx.listener(
                                                    move |this, _event, _window, cx| {
                                                        this.state.selected_memory_id = mid.clone();
                                                        this.state.navigate(Page::MemoryDetail);
                                                        cx.notify();
                                                    },
                                                ))
                                                .child(
                                                    div()
                                                        .text_size(px(12.5))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.text_primary)
                                                        .child(m.title),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(theme.text_muted)
                                                        .mt(px(4.0))
                                                        .child(format!(
                                                            "{} · {} · rank {}",
                                                            m.collection, m.mem_type, m.score
                                                        )),
                                                )
                                        }),
                                ),
                            )
                            // Divider
                            .child(div().h(px(1.0)).bg(theme.border_subtle).my(px(16.0)))
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(12.0))
                                    .child("ARCHIVE ACTIONS"),
                            )
                            .child(
                                div()
                                    .id("star-conv-btn")
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .cursor_pointer()
                                    .hover(|s| {
                                        s.bg(theme.bg_surface).border_color(theme.border_strong)
                                    })
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        if this.state.starred_conversations.contains(&conv_title) {
                                            this.state.starred_conversations.remove(&conv_title);
                                            this.state.set_toast("Removed from starred");
                                        } else {
                                            this.state
                                                .starred_conversations
                                                .insert(conv_title.clone());
                                            this.state.set_toast("Conversation starred");
                                        }
                                        cx.notify();
                                    }))
                                    .child(if is_starred {
                                        "★ Unstar Conversation"
                                    } else {
                                        "☆ Star Conversation"
                                    })
                                    .child("↗"),
                            )
                            .child(
                                div()
                                    .id("cold-archive-conv-btn")
                                    .p(px(10.0))
                                    .mt(px(8.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .cursor_pointer()
                                    .hover(|s| {
                                        s.bg(theme.bg_surface).border_color(theme.border_strong)
                                    })
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.set_toast(
                                            "Conversation flagged for cold archive tier",
                                        );
                                        cx.notify();
                                    }))
                                    .child("↧ Move to Cold Archive")
                                    .child("↗"),
                            ),
                    ),
            )
    }

    // ==========================================
    // 2. Collection Detail Screen
    // ==========================================
    pub fn render_collection_detail_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let col_name = self.state.selected_collection_name.clone();
        let collection = self
            .state
            .collections
            .iter()
            .find(|c| c.name == col_name)
            .cloned()
            .unwrap_or_else(|| self.state.collections[0].clone());

        let memories: Vec<_> = self
            .state
            .memories
            .iter()
            .filter(|m| m.collection == collection.name)
            .cloned()
            .collect();

        div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(1340.0))
            .mx_auto()
            .gap(px(16.0))
            // Head
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
                                    .child("CURATED MEMORY / COLLECTION"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child(collection.name),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child(collection.desc),
                            ),
                    )
                    .child(
                        div()
                            .id("col-back-btn")
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
                                this.state.navigate(Page::Memory);
                                cx.notify();
                            }))
                            .child("← All Collections"),
                    ),
            )
            // Badges row
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded(px(5.0))
                            .bg(theme.bg_inverse)
                            .text_color(theme.text_inverse)
                            .text_size(px(11.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(format!("{} total memories", collection.count)),
                    )
                    .child(
                        div()
                            .font_family(FONT_MONO)
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded(px(5.0))
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_subtle)
                            .text_size(px(11.5))
                            .text_color(theme.text_secondary)
                            .child(format!("Authority Density: {}%", collection.pct)),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded(px(5.0))
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_subtle)
                            .text_size(px(11.5))
                            .text_color(theme.text_secondary)
                            .child("Scoped for MCP Retrieval"),
                    ),
            )
            // Memories List Card
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
                                    .child("Curated Memories in Collection"),
                            )
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .bg(theme.bg_subtle)
                                    .text_color(theme.text_muted)
                                    .child(format!("{} shown", memories.len())),
                            ),
                    )
                    .child(div().p(px(8.0)).flex().flex_col().gap(px(4.0)).children(
                        memories.into_iter().map(|m| {
                            let mid = m.id.clone();
                            let mid_peek = m.id.clone();
                            div()
                                .id(SharedString::from(format!("col-mem-{}", m.id)))
                                .p(px(11.0))
                                .rounded(RADIUS_SM)
                                .flex()
                                .items_center()
                                .justify_between()
                                .hover(|s| s.bg(theme.bg_hover))
                                .child(
                                    div()
                                        .id(SharedString::from(format!("col-mem-cell-{}", mid)))
                                        .flex_1()
                                        .flex()
                                        .items_center()
                                        .gap(px(12.0))
                                        .cursor_pointer()
                                        .on_click(cx.listener(move |this, _event, _window, cx| {
                                            this.state.selected_memory_id = mid.clone();
                                            this.state.navigate(Page::MemoryDetail);
                                            cx.notify();
                                        }))
                                        .child(div().text_size(px(14.0)).child("◇"))
                                        .child(
                                            div()
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
                                                        .text_size(px(12.0))
                                                        .text_color(theme.text_muted)
                                                        .child(format!(
                                                            "{} · Source: {} · score:{}",
                                                            m.mem_type, m.source, m.score
                                                        )),
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
                                                .font_family(FONT_MONO)
                                                .text_size(px(11.0))
                                                .text_color(theme.text_muted)
                                                .child(m.updated),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from(format!(
                                                    "col-peek-btn-{}",
                                                    mid_peek
                                                )))
                                                .h(px(28.0))
                                                .px(px(8.0))
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
                                                .on_click(cx.listener(
                                                    move |this, _event, _window, cx| {
                                                        this.state.quick_drawer_memory_id =
                                                            Some(mid_peek.clone());
                                                        cx.notify();
                                                    },
                                                ))
                                                .child("Peek"),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from(format!(
                                                    "col-inspect-btn-{}",
                                                    m.id
                                                )))
                                                .h(px(28.0))
                                                .px(px(8.0))
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
                                                .on_click(cx.listener(
                                                    move |this, _event, _window, cx| {
                                                        this.state.selected_memory_id =
                                                            m.id.clone();
                                                        this.state.navigate(Page::MemoryDetail);
                                                        cx.notify();
                                                    },
                                                ))
                                                .child("Inspect →"),
                                        ),
                                )
                        }),
                    )),
            )
    }

    // ==========================================
    // 3. Memory Detail & Provenance Screen
    // ==========================================
    pub fn render_memory_detail_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let mem_id = self.state.selected_memory_id.clone();
        let memory = self
            .state
            .memories
            .iter()
            .find(|m| m.id == mem_id)
            .cloned()
            .unwrap_or_else(|| self.state.memories[0].clone());

        let origin_conv = memory.source.to_string();
        let collection_name = memory.collection.to_string();

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
                                    .child(format!("CURATED MEMORY / {} / {}", memory.collection.to_uppercase(), memory.id)),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child(memory.title),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child(format!("{} · Last verified {} · Hybrid Score {}", memory.mem_type, memory.updated, memory.score)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("mem-detail-back-btn")
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
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.state.selected_collection_name = collection_name.clone();
                                        this.state.navigate(Page::CollectionDetail);
                                        cx.notify();
                                    }))
                                    .child("← Back to Collection"),
                            )
                            .child(
                                div()
                                    .id("copy-mem-btn")
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
                                        this.state.set_toast("Curated memory copied to clipboard");
                                        cx.notify();
                                    }))
                                    .child("Copy Memory"),
                            ),
                    ),
            )
            // Two-column layout: Memory Content & Why Matters + Aside
            .child(
                div()
                    .flex()
                    .gap(px(18.0))
                    // Left Panel
                    .child(
                        div()
                            .flex_1()
                            .p(px(24.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .mb(px(16.0))
                                    .child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(3.0))
                                            .rounded(px(5.0))
                                            .bg(theme.bg_inverse)
                                            .text_color(theme.text_inverse)
                                            .text_size(px(11.5))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(memory.mem_type),
                                    )
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.5))
                                            .text_color(theme.text_muted)
                                            .child(format!("Verified {}", memory.updated)),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(14.5))
                                    .line_height(px(24.0))
                                    .text_color(theme.text_primary)
                                    .child(memory.content),
                            )
                            .child(div().h(px(1.0)).bg(theme.border_subtle).my(px(20.0)))
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(8.0))
                                    .child("OPERATIONAL INVARIANTS & USAGE"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(6.0))
                                    .text_size(px(13.0))
                                    .text_color(theme.text_secondary)
                                    .child("• Retain bidirectional provenance link to origin conversation.")
                                    .child("• Expose to authorized MCP clients only when collection scope is granted.")
                                    .child("• Re-evaluate authority weight if underlying project constraints are superseded."),
                            )
                            .child(div().h(px(1.0)).bg(theme.border_subtle).my(px(20.0)))
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(8.0))
                                    .child("WHY THIS MEMORY MATTERS (AUTHORITY RATIONALE)"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .line_height(px(22.0))
                                    .text_color(theme.text_secondary)
                                    .child(memory.why),
                            )
                            .child(div().h(px(1.0)).bg(theme.border_subtle).my(px(20.0)))
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(10.0))
                                    .child("SIBLING & CONNECTED MEMORIES"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .children(self.state.memories.iter().filter(|m| m.id != memory.id && (m.collection == memory.collection || m.source == memory.source)).take(3).cloned().map(|m| {
                                        let mid = m.id.clone();
                                        div()
                                            .id(SharedString::from(format!("mem-sibling-{}", m.id)))
                                            .p(px(10.0))
                                            .rounded(RADIUS_SM)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .cursor_pointer()
                                            .hover(|s| s.border_color(theme.border_focus))
                                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                                this.state.selected_memory_id = mid.clone();
                                                cx.notify();
                                            }))
                                            .child(
                                                div()
                                                    .text_size(px(12.5))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.text_primary)
                                                    .child(m.title),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(theme.text_muted)
                                                    .mt(px(4.0))
                                                    .child(format!("{} · {}", m.collection, m.mem_type)),
                                            )
                                    })),
                            ),
                    )
                    // Right Panel: Authority Record & Origin Trace
                    .child(
                        div()
                            .w(px(310.0))
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(12.0))
                                    .child("MEMORY AUTHORITY RECORD"),
                            )
                            .child(self.render_detail_meta_row("Collection", memory.collection, false))
                            .child(self.render_detail_meta_row("Record Classification", memory.mem_type, false))
                            .child(self.render_detail_meta_row("Hybrid Ranking Score", &format!("{} (BM25 + Cosine)", memory.score), false))
                            .child(self.render_detail_meta_row("Updated Timestamp", memory.updated, false))
                            .child(self.render_detail_meta_row("Vault Protection", "SQLCipher Encrypted", false))
                            .child(div().h(px(1.0)).bg(theme.border_subtle).my(px(16.0)))
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .mb(px(10.0))
                                    .child("ORIGIN CONVERSATION TRACE"),
                            )
                            .child(
                                div()
                                    .id("origin-conv-btn")
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .cursor_pointer()
                                    .hover(|s| s.border_color(theme.border_focus))
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.state.selected_conversation_title = origin_conv.clone();
                                        this.state.navigate(Page::ConversationDetail);
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_size(px(12.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child(memory.source),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(theme.text_muted)
                                            .mt(px(4.0))
                                            .child("Click to open original conversation transcript ↗"),
                                    ),
                            )
                            .child(div().h(px(1.0)).bg(theme.border_subtle).my(px(16.0)))
                            .child(
                                div()
                                    .id("pin-memory-btn")
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_surface).border_color(theme.border_strong))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.set_toast("Memory pinned to top of MCP context window");
                                        cx.notify();
                                    }))
                                    .child("☆ Pin to High-Priority Context")
                                    .child("↗"),
                            ),
                    ),
            )
    }

    fn render_detail_meta_row(
        &self,
        label: &'static str,
        value: &str,
        is_mono: bool,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        div()
            .pb(px(8.0))
            .border_b_1()
            .border_color(theme.border_subtle)
            .child(
                div()
                    .text_size(px(11.5))
                    .text_color(theme.text_muted)
                    .child(label),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.text_primary)
                    .when(is_mono, |this| this.font_family(FONT_MONO))
                    .child(value.to_string()),
            )
    }
}
