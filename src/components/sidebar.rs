use crate::models::Page;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    pub fn render_sidebar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let collapsed = self.state.sidebar_collapsed;
        let files_count = self.state.total_files();

        div()
            .flex()
            .flex_col()
            .h_full()
            .w(if collapsed { px(64.0) } else { px(256.0) })
            .min_w(if collapsed { px(64.0) } else { px(256.0) })
            .bg(theme.bg_surface)
            .border_r_1()
            .border_color(theme.border_subtle)
            .py(px(16.0))
            .px(if collapsed { px(8.0) } else { px(12.0) })
            .child(
                // Brand row
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .pb(px(16.0))
                    .px(if collapsed { px(0.0) } else { px(8.0) })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(
                                // Brand mark
                                div()
                                    .w(px(30.0))
                                    .h(px(30.0))
                                    .rounded(RADIUS_SM)
                                    .bg(theme.bg_inverse)
                                    .text_color(theme.text_inverse)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .flex_shrink_0()
                                    .child(
                                        svg()
                                            .path("icons/brand.svg")
                                            .w(px(17.0))
                                            .h(px(17.0))
                                            .text_color(theme.text_inverse),
                                    ),
                            )
                            .when(!collapsed, |this| {
                                this.child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_size(px(15.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child("Mind Place"),
                                        )
                                        .child(
                                            div()
                                                .font_family(FONT_MONO)
                                                .text_size(px(11.0))
                                                .text_color(theme.text_muted)
                                                .child("v0.9.4-local"),
                                        ),
                                )
                            }),
                    )
                    .child(
                        // Collapse button
                        div()
                            .id("toggle-sidebar-btn")
                            .w(px(26.0))
                            .h(px(26.0))
                            .rounded(RADIUS_XS)
                            .text_color(theme.text_muted)
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.state.toggle_sidebar();
                                cx.notify();
                            }))
                            .child(
                                svg()
                                    .path("icons/collapse.svg")
                                    .w(px(15.0))
                                    .h(px(15.0))
                                    .text_color(theme.text_muted),
                            ),
                    ),
            )
            .child(
                // Vault Status Pill
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .mx(px(4.0))
                    .mb(px(16.0))
                    .py(px(10.0))
                    .px(if collapsed { px(4.0) } else { px(12.0) })
                    .rounded(RADIUS_MD)
                    .bg(theme.bg_subtle)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .justify_center()
                    .child(
                        // Dot
                        div()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .bg(theme.text_primary)
                            .flex_shrink_0(),
                    )
                    .when(!collapsed, |this| {
                        this.child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child("Local Vault Unlocked"),
                                )
                                .child(
                                    div()
                                        .font_family(FONT_MONO)
                                        .text_size(px(11.0))
                                        .text_color(theme.text_muted)
                                        .child("SQLCipher · AES-256-GCM"),
                                ),
                        )
                        .child(
                            div()
                                .id("vault-lock-quick-btn")
                                .px(px(8.0))
                                .py(px(4.0))
                                .text_size(px(11.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .rounded(RADIUS_XS)
                                .border_1()
                                .border_color(theme.border_strong)
                                .bg(theme.bg_surface)
                                .text_color(theme.text_primary)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_inverse).text_color(theme.text_inverse))
                                .on_click(cx.listener(|this, _event, _window, cx| {
                                    this.state.lock_vault();
                                    cx.notify();
                                }))
                                .child("Lock"),
                        )
                    }),
            )
            // Navigation Section: Knowledge OS
            .when(!collapsed, |this| {
                this.child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .px(px(10.0))
                        .pt(px(10.0))
                        .pb(px(6.0))
                        .child("KNOWLEDGE OS"),
                )
            })
            .child(self.render_nav_item(Page::Home, "icons/nav_home.svg", "Overview", None, cx))
            .child(self.render_nav_item(
                Page::Conversations,
                "icons/nav_conversations.svg",
                "Conversations",
                Some(128),
                cx,
            ))
            .child(self.render_nav_item(
                Page::Memory,
                "icons/nav_memory.svg",
                "Curated Memory",
                Some(342),
                cx,
            ))
            .child(self.render_nav_item(
                Page::Hive,
                "icons/nav_hive.svg",
                "Hive Mind Graph",
                Some(33),
                cx,
            ))
            // Navigation Section: Authority & Infrastructure
            .when(!collapsed, |this| {
                this.child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .px(px(10.0))
                        .pt(px(14.0))
                        .pb(px(6.0))
                        .child("AUTHORITY & INFRASTRUCTURE"),
                )
            })
            .child(self.render_nav_item(
                Page::Files,
                "icons/nav_files.svg",
                "Files & Secure Vault",
                Some(files_count),
                cx,
            ))
            .child(self.render_nav_item(
                Page::Providers,
                "icons/nav_providers.svg",
                "AI Providers & MCP",
                None,
                cx,
            ))
            // Navigation Section: System
            .when(!collapsed, |this| {
                this.child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .px(px(10.0))
                        .pt(px(14.0))
                        .pb(px(6.0))
                        .child("SYSTEM"),
                )
            })
            .child(self.render_nav_item(
                Page::Setup,
                "icons/nav_setup.svg",
                "Setup & Onboarding",
                None,
                cx,
            ))
            .child(self.render_nav_item(
                Page::Settings,
                "icons/nav_settings.svg",
                "Settings",
                None,
                cx,
            ))
            // Footer
            .child(
                div()
                    .mt_auto()
                    .pt(px(12.0))
                    .border_t_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .when(!collapsed, |this| {
                        this.child(
                            // MCP status bar
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .px(px(10.0))
                                .py(px(8.0))
                                .rounded(RADIUS_SM)
                                .bg(theme.bg_subtle)
                                .child(
                                    div()
                                        .font_family(FONT_MONO)
                                        .text_size(px(11.5))
                                        .text_color(theme.text_primary)
                                        .child("MCP 127.0.0.1:43110"),
                                )
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(1.0))
                                        .rounded(RADIUS_XS)
                                        .bg(theme.bg_inverse)
                                        .text_color(theme.text_inverse)
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("ACTIVE"),
                                ),
                        )
                    })
                    .child(
                        // Profile row
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .px(if collapsed { px(4.0) } else { px(8.0) })
                            .py(px(6.0))
                            .justify_center()
                            .child(
                                div()
                                    .w(px(28.0))
                                    .h(px(28.0))
                                    .rounded_full()
                                    .bg(theme.bg_inverse)
                                    .text_color(theme.text_inverse)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::BOLD)
                                    .flex_shrink_0()
                                    .child("KA"),
                            )
                            .when(!collapsed, |this| {
                                this.child(
                                    div()
                                        .flex_1()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_size(px(12.5))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.text_primary)
                                                .child("Kais Abiyyi"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(theme.text_muted)
                                                .child("Owner · Authority Root"),
                                        ),
                                )
                            }),
                    ),
            )
    }

    fn render_nav_item(
        &mut self,
        page: Page,
        icon_path: &'static str,
        label: &'static str,
        count: Option<usize>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let collapsed = self.state.sidebar_collapsed;
        let is_active = self.state.active_page == page
            || (page == Page::Conversations && self.state.active_page == Page::ConversationDetail)
            || (page == Page::Memory
                && (self.state.active_page == Page::CollectionDetail
                    || self.state.active_page == Page::MemoryDetail));

        div()
            .id(SharedString::from(format!("nav-item-{:?}", page)))
            .h(px(36.0))
            .rounded(RADIUS_SM)
            .flex()
            .items_center()
            .gap(px(10.0))
            .px(if collapsed { px(0.0) } else { px(10.0) })
            .justify_center()
            .cursor_pointer()
            .text_size(px(13.0))
            .font_weight(if is_active {
                FontWeight::SEMIBOLD
            } else {
                FontWeight::MEDIUM
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
            .hover(move |s| {
                if !is_active {
                    s.bg(theme.bg_hover).text_color(theme.text_primary)
                } else {
                    s
                }
            })
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.state.navigate(page);
                cx.notify();
            }))
            .child(
                svg()
                    .path(icon_path)
                    .w(px(16.0))
                    .h(px(16.0))
                    .text_color(if is_active {
                        theme.text_inverse
                    } else {
                        theme.text_secondary
                    }),
            )
            .when(!collapsed, move |this| {
                this.child(
                    div()
                        .flex_1()
                        .text_color(if is_active {
                            theme.text_inverse
                        } else {
                            theme.text_secondary
                        })
                        .child(label),
                )
                .when_some(count, move |this, count_val| {
                    this.child(
                        div()
                            .font_family(FONT_MONO)
                            .text_size(px(11.0))
                            .px(px(6.0))
                            .py(px(1.0))
                            .rounded_full()
                            .bg(if is_active {
                                rgba(0xFFFFFF28)
                            } else {
                                theme.bg_subtle
                            })
                            .text_color(if is_active {
                                theme.text_inverse
                            } else {
                                theme.text_secondary
                            })
                            .child(count_val.to_string()),
                    )
                })
            })
    }
}
