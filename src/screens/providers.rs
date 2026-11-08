use crate::models::DialogModal;
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::*;

impl MindPlaceApp {
    pub fn render_providers_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;

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
                                    .child("INTEGRATION-CORE · MODEL CONTEXT PROTOCOL (MCP)"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("AI Providers & MCP Authorization"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child("Grant granular, revocable read/write capabilities per AI client. Every provider operates under Default-Deny isolation."),
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
                            .child("MCP Transport: stdio + local HTTP"),
                    ),
            )
            // 3 Provider Cards
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .child(self.render_provider_card("ChatGPT", "OpenAI · Native Host + MCP", "✳", "Authorized reader and reasoning partner for your granted Mind Place collections.", cx))
                    .child(self.render_provider_card("Claude", "Anthropic · Desktop MCP", "✺", "Connect Claude Desktop via local stdio MCP transport to query engineering & research roots.", cx))
                    .child(self.render_provider_card("Gemini", "Google · Browser Adapter", "✧", "Optional browser capture and scoped retrieval bridge for Gemini workspace sessions.", cx)),
            )
            // Bottom Two Columns: Zero-Trust Architecture & Tool Call Audit Log
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    // Panel 1: Zero-Trust Architecture
                    .child(
                        div()
                            .flex_1()
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).child("Zero-Trust Permission Architecture"))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mb(px(16.0)).child("Enforced by crates/integration-core/src/clients.rs"))
                            .child(
                                div()
                                    .py(px(12.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Default-Deny Client Registration"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Unrecognized MCP clients cannot list collections or invoke retrieval tools.")),
                                    )
                                    .child(self.render_status_badge("ENFORCED", true)),
                            )
                            .child(
                                div()
                                    .py(px(12.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Collection-Level Access Scope"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Each AI provider can be restricted to specific collections (e.g., Engineering only).")),
                                    )
                                    .child(self.render_status_badge("ENFORCED", true)),
                            )
                            .child(
                                div()
                                    .py(px(12.0))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Instant Cryptographic Revocation Fence"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Revoking a client immediately invalidates its session token and blocks in-flight reads.")),
                                    )
                                    .child(self.render_status_badge("READY", true)),
                            ),
                    )
                    // Panel 2: Tool Call Audit Log
                    .child(
                        div()
                            .w(px(520.0))
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).child("Recent MCP Tool Call Audit Log"))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mb(px(16.0)).child("Real-time trace of external AI agents querying your Shared-Brain."))
                            .child(
                                div()
                                    .p(px(12.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.5))
                                    .text_color(theme.text_secondary)
                                    .flex()
                                    .flex_col()
                                    .gap(px(6.0))
                                    .child("[21:41:02] client=ChatGPT tool=retrieve_context scope=Engineering latency=4.1ms status=200_OK")
                                    .child("[21:38:19] client=ChatGPT tool=fetch_exact_source id=conv_bomc_02 latency=1.9ms status=200_OK")
                                    .child("[20:15:44] client=Claude tool=write_memory_proposal status=DENIED_WRITE_OFF")
                                    .child("[19:02:11] client=Gemini tool=list_collections status=DENIED_DISCONNECTED"),
                            ),
                    ),
            )
    }

    fn render_provider_card(
        &mut self,
        name: &'static str,
        company: &'static str,
        symbol: &'static str,
        detail: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_conn = *self.state.connected_providers.get(name).unwrap_or(&false);
        let permissions = *self
            .state
            .provider_permissions
            .get(name)
            .unwrap_or(&(false, false));
        let read_on = permissions.0;
        let write_on = permissions.1;

        div()
            .id(SharedString::from(format!("provider-card-{}", name)))
            .flex_1()
            .p(px(20.0))
            .rounded(RADIUS_MD)
            .border_1()
            .border_color(theme.border_subtle)
            .bg(theme.bg_surface)
            .flex()
            .flex_col()
            .justify_between()
            .child(
                div()
                    .child(
                        // Header
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .mb(px(14.0))
                            .child(
                                div()
                                    .w(px(38.0))
                                    .h(px(38.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(16.0))
                                    .text_color(theme.text_primary)
                                    .child(symbol),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(div().text_size(px(15.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(name))
                                    .child(div().text_size(px(12.0)).text_color(theme.text_muted).child(company)),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded(px(5.0))
                                    .bg(if is_conn { theme.bg_inverse } else { theme.bg_subtle })
                                    .text_color(if is_conn { theme.text_inverse } else { theme.text_secondary })
                                    .border_1()
                                    .border_color(if is_conn { theme.bg_inverse } else { theme.border_subtle })
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(if is_conn { "CONNECTED" } else { "DISCONNECTED" }),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(12.5))
                            .text_color(theme.text_secondary)
                            .mb(px(16.0))
                            .child(detail),
                    )
                    .child(
                        div()
                            .id(SharedString::from(format!("toggle-conn-{}", name)))
                            .w_full()
                            .h(px(36.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(if is_conn { theme.border_strong } else { theme.bg_inverse })
                            .bg(if is_conn { theme.bg_surface } else { theme.bg_inverse })
                            .text_color(if is_conn { theme.text_primary } else { theme.text_inverse })
                            .text_size(px(12.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                let cur = *this.state.connected_providers.get(name).unwrap_or(&false);
                                this.state.connected_providers.insert(name.to_string(), !cur);
                                this.state.set_toast(format!("{} {}", name, if !cur { "authorized via MCP" } else { "revoked and fenced" }));
                                cx.notify();
                            }))
                            .child(if is_conn {
                                "Revoke & Disconnect Provider"
                            } else {
                                "Authorize Provider via MCP"
                            }),
                    ),
            )
            .child(
                div()
                    .pt(px(14.0))
                    .border_t_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    // Read Capability Switch
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).child("MCP Read Capability"))
                                    .child(div().text_size(px(11.5)).text_color(theme.text_secondary).child("Retrieve context from scoped collections.")),
                            )
                            .child(
                                div()
                                    .id(SharedString::from(format!("prov-read-{}", name)))
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        let mut perms = *this.state.provider_permissions.get(name).unwrap_or(&(false, false));
                                        perms.0 = !perms.0;
                                        this.state.provider_permissions.insert(name.to_string(), perms);
                                        this.state.set_toast("Capability boundary updated");
                                        cx.notify();
                                    }))
                                    .child(self.render_switch(read_on)),
                            ),
                    )
                    // Write Capability Switch
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).child("MCP Write Proposals"))
                                    .child(div().text_size(px(11.5)).text_color(theme.text_secondary).child("Propose curated memories (Default OFF).")),
                            )
                            .child(
                                div()
                                    .id(SharedString::from(format!("prov-write-{}", name)))
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        let mut perms = *this.state.provider_permissions.get(name).unwrap_or(&(false, false));
                                        perms.1 = !perms.1;
                                        this.state.provider_permissions.insert(name.to_string(), perms);
                                        this.state.set_toast("Capability boundary updated");
                                        cx.notify();
                                    }))
                                    .child(self.render_switch(write_on)),
                            ),
                    )
                    // Scope ACL
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).child("Collection & Folder Scope"))
                                    .child(div().text_size(px(11.5)).text_color(theme.text_secondary).child(format!("{} collections · {} folders", self.state.collections.len(), self.state.approved_folders.len()))),
                            )
                            .child(
                                div()
                                    .id(SharedString::from(format!("scope-acl-{}", name)))
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
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.state.dialog_modal = Some(DialogModal {
                                            kicker: "INTEGRATION-CORE · ACL SCOPE".into(),
                                            title: format!("{} — Collection & Folder Scope Matrix", name),
                                            body_html: format!("Select which memory collections and authorized local folders {} is permitted to read during MCP tool invocations:\n\n• Engineering (142 memories) — GRANTED\n• Research (87 memories) — GRANTED\n• Learning (56 memories) — GRANTED\n• Life & goals (34 memories) — DENIED\n• Product ideas (18 memories) — DENIED", name),
                                        });
                                        cx.notify();
                                    }))
                                    .child("Scope ACL →"),
                            ),
                    ),
            )
    }

    pub fn render_status_badge(&self, label: &'static str, solid: bool) -> impl IntoElement {
        let theme = self.state.theme;
        div()
            .font_family(FONT_MONO)
            .text_size(px(11.0))
            .font_weight(FontWeight::BOLD)
            .px(px(8.0))
            .py(px(3.0))
            .rounded(px(4.0))
            .bg(if solid {
                theme.bg_inverse
            } else {
                theme.bg_subtle
            })
            .text_color(if solid {
                theme.text_inverse
            } else {
                theme.text_secondary
            })
            .child(label)
    }
}
