use crate::models::Page;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    pub fn render_overview_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let tech_mode = self.state.tech_mode;
        let total_files = self.state.total_files();
        let provider_count = self.state.connected_provider_count();

        div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(1340.0))
            .mx_auto()
            .gap(px(20.0))
            // Page Header
            .child(
                div()
                    .flex()
                    .items_end()
                    .justify_between()
                    .gap(px(20.0))
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
                                    .child("PERSONAL KNOWLEDGE OS · SHARED-BRAIN RUNTIME"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("Workspace Command Center"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child("Local-first intelligence archive unifying AI conversations, curated memories, and NTFS document roots."),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("quick-inspect-overview-btn")
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
                                        this.state.quick_drawer_memory_id = Some("Engineering-0".into());
                                        cx.notify();
                                    }))
                                    .child("Inspect Latest Memory"),
                            )
                            .child(
                                div()
                                    .id("capture-new-thread-btn")
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
                                        this.state.navigate(Page::Conversations);
                                        cx.notify();
                                    }))
                                    .child("＋ Capture / New Thread"),
                            ),
                    ),
            )
            // Status strip
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(16.0))
                    .p(px(16.0))
                    .rounded(RADIUS_MD)
                    .border_1()
                    .border_color(theme.border_strong)
                    .bg(theme.bg_surface)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(14.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(7.0))
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded_full()
                                    .bg(theme.bg_inverse)
                                    .text_color(theme.text_inverse)
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(
                                        div()
                                            .w(px(6.0))
                                            .h(px(6.0))
                                            .rounded_full()
                                            .bg(theme.text_inverse),
                                    )
                                    .child("GEN-14 GRAPH COHERENT"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child("Workspace setup is 66% complete (Step 2 of 3)"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.text_secondary)
                                            .child("Browser capture host active · 1 of 3 AI providers authorized via MCP"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .when(tech_mode, |this| {
                                this.child(
                                    div()
                                        .font_family(FONT_MONO)
                                        .text_size(px(11.5))
                                        .text_color(theme.text_muted)
                                        .child("WAL: checkpoint_ok · 4.1ms"),
                                )
                            })
                            .child(
                                div()
                                    .id("onboarding-btn")
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
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.navigate(Page::Setup);
                                        cx.notify();
                                    }))
                                    .child("Complete Onboarding →"),
                            ),
                    ),
            )
            // 4 Stats grid
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .child(self.render_stat_card("Archived Conversations", "128", "+6 captured this week", "▤", "archive-core", Page::Conversations, cx))
                    .child(self.render_stat_card("Curated Memories", "342", "+24 ranked insights", "◇", "state-core v9", Page::Memory, cx))
                    .child(self.render_stat_card("Secure Vault & NTFS Objects", &total_files.to_string(), "3 folders · AES-256 fenced", "▣", "security-core", Page::Files, cx))
                    .child(self.render_stat_card("Authorized AI Providers", &format!("{} / 3", provider_count), "ChatGPT read-scoped", "⌘", "mcp-v1", Page::Providers, cx)),
            )
            // Home bottom grid: Recent Conversations (1.3fr), Collections (0.9fr), Operations (280px)
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    // Column 1: Recent Conversations
                    .child(
                        div()
                            .flex_1()
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
                                            .text_color(theme.text_primary)
                                            .child("Recent Conversations"),
                                    )
                                    .child(
                                        div()
                                            .id("view_all_convs_btn")
                                            .text_size(px(12.0))
                                            .text_color(theme.text_secondary)
                                            .cursor_pointer()
                                            .hover(|s| s.text_color(theme.text_primary))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.navigate(Page::Conversations);
                                                cx.notify();
                                            }))
                                            .child("View all 128 →"),
                                    ),
                            )
                            .child(
                                div()
                                    .p(px(8.0))
                                    .flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .children(self.state.conversations.iter().take(5).cloned().map(|c| {
                                        let title = c.title.to_string();
                                        div()
                                            .id(SharedString::from(format!("home-conv-{}", c.title)))
                                            .flex()
                                            .items_center()
                                            .gap(px(12.0))
                                            .p(px(10.0))
                                            .rounded(RADIUS_SM)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.bg_hover))
                                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                                this.state.selected_conversation_title = title.clone();
                                                this.state.navigate(Page::ConversationDetail);
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
                                                    .child("▤"),
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
                                                            .child(c.title),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(8.0))
                                                            .text_size(px(12.0))
                                                            .text_color(theme.text_muted)
                                                            .child(c.provider)
                                                            .child("·")
                                                            .child(c.collection)
                                                            .when(tech_mode, |this| {
                                                                this.child("·").child(
                                                                    div()
                                                                        .font_family(FONT_MONO)
                                                                        .child(c.sha),
                                                                )
                                                            }),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.text_muted)
                                                    .child(c.timestamp),
                                            )
                                    })),
                            ),
                    )
                    // Column 2: Memory Collections
                    .child(
                        div()
                            .w(px(360.0))
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
                                            .text_color(theme.text_primary)
                                            .child("Memory Collections"),
                                    )
                                    .child(
                                        div()
                                            .id("open_memory_lib_btn")
                                            .text_size(px(12.0))
                                            .text_color(theme.text_secondary)
                                            .cursor_pointer()
                                            .hover(|s| s.text_color(theme.text_primary))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.navigate(Page::Memory);
                                                cx.notify();
                                            }))
                                            .child("Open Library →"),
                                    ),
                            )
                            .child(
                                div()
                                    .p(px(8.0))
                                    .flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .children(self.state.collections.iter().take(5).cloned().map(|col| {
                                        let col_name = col.name.to_string();
                                        div()
                                            .id(SharedString::from(format!("home-col-{}", col.name)))
                                            .flex()
                                            .items_center()
                                            .gap(px(12.0))
                                            .p(px(10.0))
                                            .rounded(RADIUS_SM)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.bg_hover))
                                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                                this.state.selected_collection_name = col_name.clone();
                                                this.state.navigate(Page::CollectionDetail);
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
                                                    .flex_1()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_between()
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.5))
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(theme.text_primary)
                                                                    .child(col.name),
                                                            )
                                                            .child(
                                                                div()
                                                                    .font_family(FONT_MONO)
                                                                    .text_size(px(11.0))
                                                                    .text_color(theme.text_muted)
                                                                    .child(format!("{} units", col.count)),
                                                            ),
                                                    )
                                                    .child(
                                                        // Progress bar
                                                        div()
                                                            .h(px(4.0))
                                                            .w_full()
                                                            .rounded_full()
                                                            .bg(theme.bg_muted)
                                                            .mt(px(8.0))
                                                            .overflow_hidden()
                                                            .child(
                                                                div()
                                                                    .h_full()
                                                                    .w(px((col.pct as f32) * 2.5))
                                                                    .rounded_full()
                                                                    .bg(theme.text_primary),
                                                            ),
                                                    ),
                                            )
                                    })),
                            ),
                    )
                    // Column 3: Operations & Telemetry
                    .child(
                        div()
                            .w(px(300.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .rounded(RADIUS_MD)
                            .p(px(20.0))
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child("Operations & Telemetry"),
                            )
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .text_color(theme.text_secondary)
                                    .mb(px(16.0))
                                    .child("Direct workspace actions & ingestion feed."),
                            )
                            // Quick Action 1
                            .child(
                                div()
                                    .id("home-quick-add-folder")
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_primary)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_surface).border_color(theme.border_strong))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.add_folder_modal_open = true;
                                        cx.notify();
                                    }))
                                    .child("＋ Authorize Local Folder")
                                    .child(div().font_family(FONT_MONO).child("roots →")),
                            )
                            // Quick Action 2
                            .child(
                                div()
                                    .id("home-quick-explore-hive")
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .p(px(10.0))
                                    .my(px(8.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_primary)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_surface).border_color(theme.border_strong))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.navigate(Page::Hive);
                                        cx.notify();
                                    }))
                                    .child("Explore Hive Mind Graph")
                                    .child(div().font_family(FONT_MONO).child("33 nodes →")),
                            )
                            // Quick Action 3
                            .child(
                                div()
                                    .id("home-quick-mcp-scopes")
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_primary)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_surface).border_color(theme.border_strong))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.navigate(Page::Providers);
                                        cx.notify();
                                    }))
                                    .child("Configure MCP Client Scopes")
                                    .child(div().font_family(FONT_MONO).child("ACL →")),
                            )
                            // Telemetry box
                            .child(
                                div()
                                    .mt_auto()
                                    .pt(px(14.0))
                                    .border_t_1()
                                    .border_color(theme.border_subtle)
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_muted)
                                            .mb(px(8.0))
                                            .child("LIVE RUNTIME TELEMETRY"),
                                    )
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .text_color(theme.text_secondary)
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .child("[21:42:10] capture-host: ChatGPT DOM diff +2 msgs")
                                            .child("[21:42:11] index-core: FTS5 + BM25 re-ranked (3.8ms)")
                                            .child("[21:40:04] security-core: SQLCipher page integrity OK"),
                                    ),
                            ),
                    ),
            )
    }

    fn render_stat_card(
        &mut self,
        title: &'static str,
        value: &str,
        footer: &'static str,
        icon: &'static str,
        tech_label: &'static str,
        target_page: Page,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let tech_mode = self.state.tech_mode;

        div()
            .id(SharedString::from(format!("stat-card-{}", title)))
            .flex_1()
            .p(px(18.0))
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
                this.state.navigate(target_page);
                cx.notify();
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_size(px(12.5))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.text_secondary)
                    .child(title)
                    .child(
                        div()
                            .w(px(30.0))
                            .h(px(30.0))
                            .rounded(RADIUS_SM)
                            .bg(theme.bg_subtle)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .text_color(theme.text_primary)
                            .child(icon),
                    ),
            )
            .child(
                div()
                    .text_size(px(28.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .my(px(8.0))
                    .child(value.to_string()),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_size(px(12.0))
                    .text_color(theme.text_muted)
                    .child(footer)
                    .when(tech_mode, |this| {
                        this.child(
                            div()
                                .font_family(FONT_MONO)
                                .text_color(theme.text_muted)
                                .child(tech_label),
                        )
                    }),
            )
    }
}
