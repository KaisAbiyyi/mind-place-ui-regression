use crate::models::Page;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::*;

impl MindPlaceApp {
    pub fn render_setup_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let step = self.state.setup_step;

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
                                    .child("FIRST-RUN QUALIFICATION WIZARD"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("Set up Mind Place"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child("A guided, local-first onboarding flow from vault initialization to your first verified knowledge graph."),
                            ),
                    )
                    .child(
                        div()
                            .font_family(FONT_MONO)
                            .text_size(px(11.0))
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(5.0))
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_subtle)
                            .text_color(theme.text_secondary)
                            .child(format!("Step {} of 4", step + 1)),
                    ),
            )
            // Setup Layout: Steps Sidebar + Panel
            .child(
                div()
                    .flex()
                    .gap(px(18.0))
                    // Steps Sidebar
                    .child(
                        div()
                            .w(px(250.0))
                            .p(px(16.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(self.render_step_btn(0, "Welcome & Architecture", cx))
                            .child(self.render_step_btn(1, "Initialize Secure Vault", cx))
                            .child(self.render_step_btn(2, "Authorize AI Providers", cx))
                            .child(self.render_step_btn(3, "First Knowledge Result", cx)),
                    )
                    // Setup Content Panel
                    .child(
                        div()
                            .flex_1()
                            .min_h(px(440.0))
                            .p(px(24.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .flex()
                            .flex_col()
                            .justify_between()
                            .child(self.render_setup_panel_content(step, cx)),
                    ),
            )
    }

    fn render_step_btn(
        &mut self,
        step_idx: usize,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_active = self.state.setup_step == step_idx;
        let is_done = self.state.setup_step > step_idx;

        div()
            .id(SharedString::from(format!("setup-step-btn-{}", step_idx)))
            .p(px(10.0))
            .rounded(RADIUS_SM)
            .flex()
            .items_center()
            .gap(px(10.0))
            .cursor_pointer()
            .bg(if is_active {
                theme.bg_inverse
            } else {
                theme.bg_surface
            })
            .text_color(if is_active {
                theme.text_inverse
            } else if is_done {
                theme.text_primary
            } else {
                theme.text_secondary
            })
            .hover(move |s| if !is_active { s.bg(theme.bg_hover) } else { s })
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.state.setup_step = step_idx;
                cx.notify();
            }))
            .child(
                div()
                    .w(px(22.0))
                    .h(px(22.0))
                    .rounded_full()
                    .border_1()
                    .border_color(if is_active {
                        theme.text_inverse
                    } else {
                        theme.text_secondary
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .font_family(FONT_MONO)
                    .text_size(px(11.0))
                    .child(if is_done {
                        "✓"
                    } else {
                        match step_idx {
                            0 => "1",
                            1 => "2",
                            2 => "3",
                            _ => "4",
                        }
                    }),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(if is_active {
                        FontWeight::SEMIBOLD
                    } else {
                        FontWeight::MEDIUM
                    })
                    .child(label),
            )
    }

    fn render_setup_panel_content(
        &mut self,
        step: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;

        match step {
            0 => {
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .h_full()
                    .child(
                        div()
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("STEP 01 · ARCHITECTURE OVERVIEW"))
                            .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).my(px(8.0)).child("Welcome to Mind Place."))
                            .child(div().text_size(px(13.5)).text_color(theme.text_secondary).max_w(px(600.0)).child("Your AI conversations, local documents, and evolving thoughts deserve a single, private, local-first home that works across every AI tool you use."))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .mt(px(24.0))
                                    .child(
                                        div()
                                            .flex_1()
                                            .p(px(16.0))
                                            .rounded(RADIUS_SM)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .child(div().text_size(px(18.0)).child("▣"))
                                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).mt(px(10.0)).child("1. Local Vault"))
                                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mt(px(4.0)).child("SQLCipher encrypted storage under your physical control.")),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .p(px(16.0))
                                            .rounded(RADIUS_SM)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .child(div().text_size(px(18.0)).child("⌘"))
                                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).mt(px(10.0)).child("2. Scoped MCP"))
                                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mt(px(4.0)).child("Grant ChatGPT, Claude, or Gemini granular read access.")),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .p(px(16.0))
                                            .rounded(RADIUS_SM)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .child(div().text_size(px(18.0)).child("◇"))
                                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).mt(px(10.0)).child("3. Hive Topology"))
                                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mt(px(4.0)).child("Trace every insight back to its exact source message.")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .pt(px(18.0))
                            .border_t_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("Estimated setup time: 90 seconds"))
                            .child(
                                div()
                                    .id("setup-next-0")
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
                                        this.state.setup_step = 1;
                                        cx.notify();
                                    }))
                                    .child("Initialize Vault →"),
                            ),
                    )
            }
            1 => {
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .h_full()
                    .child(
                        div()
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("STEP 02 · SECURITY-CORE INITIALIZATION"))
                            .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).my(px(8.0)).child("Configure Your Encrypted Vault"))
                            .child(div().text_size(px(13.5)).text_color(theme.text_secondary).max_w(px(600.0)).child("Set your master passphrase to derive the 256-bit SQLCipher key. All transcripts, memories, and FTS5 indexes are fenced inside this boundary."))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(18.0))
                                    .mt(px(20.0))
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .flex_col()
                                            .gap(px(12.0))
                                            .child(
                                                div()
                                                    .child(div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD).mb(px(4.0)).child("Vault Identifier"))
                                                    .child(
                                                        div()
                                                            .h(px(36.0))
                                                            .px(px(12.0))
                                                            .rounded(RADIUS_SM)
                                                            .border_1()
                                                            .border_color(theme.border_strong)
                                                            .bg(theme.bg_surface)
                                                            .flex()
                                                            .items_center()
                                                            .text_size(px(13.0))
                                                            .child("primary-local-vault"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .child(div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD).mb(px(4.0)).child("Owner Passphrase"))
                                                    .child(
                                                        div()
                                                            .h(px(36.0))
                                                            .px(px(12.0))
                                                            .rounded(RADIUS_SM)
                                                            .border_1()
                                                            .border_color(theme.border_strong)
                                                            .bg(theme.bg_surface)
                                                            .flex()
                                                            .items_center()
                                                            .text_size(px(13.0))
                                                            .child("••••••••••••••••"),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .w(px(280.0))
                                            .p(px(16.0))
                                            .rounded(RADIUS_SM)
                                            .bg(theme.bg_subtle)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .child(div().text_size(px(13.5)).font_weight(FontWeight::SEMIBOLD).mb(px(8.0)).child("Protected Subsystems"))
                                            .child(div().py(px(6.0)).flex().justify_between().child("Conversation Archive").child(div().px(px(6.0)).py(px(1.0)).rounded(px(4.0)).bg(theme.bg_inverse).text_color(theme.text_inverse).font_family(FONT_MONO).text_size(px(10.5)).child("AES-256")))
                                            .child(div().py(px(6.0)).flex().justify_between().child("Curated Memories").child(div().px(px(6.0)).py(px(1.0)).rounded(px(4.0)).bg(theme.bg_inverse).text_color(theme.text_inverse).font_family(FONT_MONO).text_size(px(10.5)).child("AES-256")))
                                            .child(div().py(px(6.0)).flex().justify_between().child("NTFS Document Objects").child(div().px(px(6.0)).py(px(1.0)).rounded(px(4.0)).bg(theme.bg_inverse).text_color(theme.text_inverse).font_family(FONT_MONO).text_size(px(10.5)).child("AES-256"))),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .pt(px(18.0))
                            .border_t_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .id("setup-back-step1-btn")
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
                                        this.state.setup_step = 0;
                                        cx.notify();
                                    }))
                                    .child("← Back"),
                            )
                            .child(
                                div()
                                    .id("setup-next-1")
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
                                        this.state.setup_step = 2;
                                        cx.notify();
                                    }))
                                    .child("Continue to AI Providers →"),
                            ),
                    )
            }
            2 => {
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .h_full()
                    .child(
                        div()
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("STEP 03 · INTEGRATION-CORE"))
                            .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).my(px(8.0)).child("Authorize AI Providers"))
                            .child(div().text_size(px(13.5)).text_color(theme.text_secondary).max_w(px(600.0)).child("Choose which AI providers can connect to your local MCP endpoint. Every provider starts with Default-Deny permissions."))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(10.0))
                                    .mt(px(18.0))
                                    .child(self.render_setup_prov_row("ChatGPT", "OpenAI", "✳", cx))
                                    .child(self.render_setup_prov_row("Claude", "Anthropic", "✺", cx))
                                    .child(self.render_setup_prov_row("Gemini", "Google", "✧", cx)),
                            ),
                    )
                    .child(
                        div()
                            .pt(px(18.0))
                            .border_t_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .id("setup-back-step1-btn")
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
                                        this.state.setup_step = 1;
                                        cx.notify();
                                    }))
                                    .child("← Back"),
                            )
                            .child(
                                div()
                                    .id("setup-next-2")
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
                                        this.state.setup_step = 3;
                                        cx.notify();
                                    }))
                                    .child("Verify First Result →"),
                            ),
                    )
            }
            _ => {
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .h_full()
                    .child(
                        div()
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("STEP 04 · READY FOR OPERATION"))
                            .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).my(px(8.0)).child("Your Knowledge OS is Online."))
                            .child(div().text_size(px(13.5)).text_color(theme.text_secondary).max_w(px(600.0)).child("All local crates are initialized. Jump directly into your conversation archive or explore the spatial Hive Mind topology."))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .mt(px(24.0))
                                    .child(
                                        div()
                                            .id("setup-open-convs-card")
                                            .flex_1()
                                            .p(px(20.0))
                                            .rounded(RADIUS_MD)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_surface)
                                            .cursor_pointer()
                                            .hover(|s| s.border_color(theme.border_strong))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.navigate(Page::Conversations);
                                                cx.notify();
                                            }))
                                            .child(div().text_size(px(20.0)).child("▤"))
                                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).mt(px(10.0)).child("Open Conversations Archive"))
                                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mt(px(4.0)).child("Inspect 128 captured multi-provider transcripts.")),
                                    )
                                    .child(
                                        div()
                                            .id("setup-launch-hive-card")
                                            .flex_1()
                                            .p(px(20.0))
                                            .rounded(RADIUS_MD)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_surface)
                                            .cursor_pointer()
                                            .hover(|s| s.border_color(theme.border_strong))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.navigate(Page::Hive);
                                                cx.notify();
                                            }))
                                            .child(div().text_size(px(20.0)).child("⌘"))
                                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).mt(px(10.0)).child("Launch Hive Mind Graph"))
                                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mt(px(4.0)).child("Navigate 33 concept clusters and 67 provenance links.")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .pt(px(18.0))
                            .border_t_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .id("setup-back-step3-btn")
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
                                        this.state.setup_step = 2;
                                        cx.notify();
                                    }))
                                    .child("← Back"),
                            )
                            .child(
                                div()
                                    .id("setup-finish-btn")
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
                                        this.state.navigate(Page::Home);
                                        cx.notify();
                                    }))
                                    .child("Finish & Open Command Center →"),
                            ),
                    )
            }
        }
    }

    fn render_setup_prov_row(
        &mut self,
        name: &'static str,
        comp: &'static str,
        sym: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_conn = *self.state.connected_providers.get(name).unwrap_or(&false);

        div()
            .p(px(14.0))
            .rounded(RADIUS_SM)
            .border_1()
            .border_color(theme.border_subtle)
            .bg(theme.bg_subtle)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(14.0))
                            .text_color(theme.text_primary)
                            .child(sym),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child(name),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.text_muted)
                                    .child(format!(
                                        "{} · {}",
                                        comp,
                                        if is_conn {
                                            "Authorized (Read-Scoped)"
                                        } else {
                                            "Disconnected"
                                        }
                                    )),
                            ),
                    ),
            )
            .child(
                div()
                    .id(SharedString::from(format!("setup-prov-btn-{}", name)))
                    .h(px(30.0))
                    .px(px(12.0))
                    .rounded(RADIUS_SM)
                    .bg(if is_conn {
                        theme.bg_surface
                    } else {
                        theme.bg_inverse
                    })
                    .text_color(if is_conn {
                        theme.text_primary
                    } else {
                        theme.text_inverse
                    })
                    .border_1()
                    .border_color(if is_conn {
                        theme.border_strong
                    } else {
                        theme.bg_inverse
                    })
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _event, _window, cx| {
                        let cur = *this.state.connected_providers.get(name).unwrap_or(&false);
                        this.state
                            .connected_providers
                            .insert(name.to_string(), !cur);
                        cx.notify();
                    }))
                    .child(if is_conn { "Disconnect" } else { "Authorize" }),
            )
    }
}
