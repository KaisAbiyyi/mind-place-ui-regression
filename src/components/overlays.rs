use crate::models::{ApprovedFolder, Page, SampleFile};
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    pub fn render_overlays(&mut self, cx: &mut Context<Self>) -> Vec<impl IntoElement> {
        let mut overlays = Vec::new();

        if self.state.vault_locked {
            overlays.push(self.render_vault_lock_screen(cx).into_any_element());
        }

        if self.state.cmd_palette_open {
            overlays.push(self.render_command_palette(cx).into_any_element());
        }

        if self.state.quick_drawer_memory_id.is_some()
            || self.state.quick_drawer_folder_id.is_some()
        {
            overlays.push(self.render_slide_drawer(cx).into_any_element());
        }

        if self.state.add_folder_modal_open {
            overlays.push(self.render_add_folder_modal(cx).into_any_element());
        }

        if self.state.dialog_modal.is_some() {
            overlays.push(self.render_generic_modal(cx).into_any_element());
        }

        if let Some((msg, _)) = &self.state.toast {
            overlays.push(self.render_toast_pill(msg.clone(), cx).into_any_element());
        }

        overlays
    }

    fn render_vault_lock_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;

        div()
            .absolute()
            .inset_0()
            .bg(theme.bg_app)
            .flex()
            .items_center()
            .justify_center()
            .p(px(24.0))
            .child(
                div()
                    .w(px(420.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_strong)
                    .rounded(RADIUS_MD)
                    .p(px(32.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .w(px(42.0))
                            .h(px(42.0))
                            .rounded(RADIUS_SM)
                            .bg(theme.bg_inverse)
                            .text_color(theme.text_inverse)
                            .flex()
                            .items_center()
                            .justify_center()
                            .mb(px(16.0))
                            .child(
                                svg()
                                    .path("icons/lock.svg")
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .text_color(theme.text_inverse),
                            ),
                    )
                    .child(
                        div()
                            .font_family(FONT_MONO)
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_muted)
                            .child("MP-3L1 MANUAL LOCK ENFORCED"),
                    )
                    .child(
                        div()
                            .text_size(px(22.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .my(px(8.0))
                            .child("Vault is Locked"),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.text_secondary)
                            .mb(px(20.0))
                            .child("In-memory SQLCipher keys have been zeroized and MCP client requests are fenced. Enter your owner passphrase to restore workspace state."),
                    )
                    .child(
                        div()
                            .w_full()
                            .h(px(36.0))
                            .px(px(12.0))
                            .rounded(RADIUS_SM)
                            .border_1()
                            .border_color(theme.border_strong)
                            .bg(theme.bg_surface)
                            .text_color(theme.text_primary)
                            .flex()
                            .items_center()
                            .justify_center()
                            .mb(px(12.0))
                            .child("••••••••••••••••"),
                    )
                    .child(
                        div()
                            .id("unlock-vault-btn")
                            .w_full()
                            .h(px(36.0))
                            .rounded(RADIUS_SM)
                            .bg(theme.bg_inverse)
                            .text_color(theme.text_inverse)
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_size(px(13.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.state.unlock_vault();
                                cx.notify();
                            }))
                            .child("Unlock Local Vault →"),
                    ),
            )
    }

    fn render_command_palette(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let query = self.state.cmd_query.trim().to_lowercase();

        // Build list of matching items
        struct CmdItem {
            kind: &'static str,
            label: String,
            sub: String,
            action_code: usize, // 0: add folder, 1: toggle theme, 2: toggle tech, 3: lock vault, 10: conv, 20: memory, 30: hive, 40: folder
            target: String,
        }

        let mut items = vec![
            CmdItem {
                kind: "ACTION",
                label: "Authorize New Local Folder (＋ Add Allowed Folder)".into(),
                sub: "Files & Secure Vault · state-core::document_roots".into(),
                action_code: 0,
                target: String::new(),
            },
            CmdItem {
                kind: "ACTION",
                label: "Toggle Dark / Light Monochrome Theme (⌘D)".into(),
                sub: "Appearance".into(),
                action_code: 1,
                target: String::new(),
            },
            CmdItem {
                kind: "ACTION",
                label: "Toggle Technical Provenance Mode (⌘I)".into(),
                sub: "Developer Telemetry".into(),
                action_code: 2,
                target: String::new(),
            },
            CmdItem {
                kind: "ACTION",
                label: "Lock Secure Vault Immediately (MP-3L1)".into(),
                sub: "Security-Core".into(),
                action_code: 3,
                target: String::new(),
            },
        ];

        for folder in &self.state.approved_folders {
            items.push(CmdItem {
                kind: "FOLDER ROOT",
                label: folder.path.clone(),
                sub: format!(
                    "{} · {} files ({}) · {}",
                    folder.collection,
                    folder.files_count,
                    folder.formats.join(", "),
                    folder.status
                ),
                action_code: 40,
                target: folder.id.clone(),
            });
        }

        for conv in &self.state.conversations {
            items.push(CmdItem {
                kind: "CONVERSATION",
                label: conv.title.to_string(),
                sub: format!(
                    "{} · {} · {}",
                    conv.provider, conv.collection, conv.timestamp
                ),
                action_code: 10,
                target: conv.title.to_string(),
            });
        }

        for mem in &self.state.memories {
            items.push(CmdItem {
                kind: "MEMORY",
                label: mem.title.to_string(),
                sub: format!("{} · {}", mem.collection, mem.mem_type),
                action_code: 20,
                target: mem.id.clone(),
            });
        }

        for node in &self.state.hive_nodes {
            items.push(CmdItem {
                kind: "HIVE NODE",
                label: node.title.to_string(),
                sub: format!("{} · {} traces", node.tag, node.sources),
                action_code: 30,
                target: node.id.to_string(),
            });
        }

        let filtered: Vec<CmdItem> = items
            .into_iter()
            .filter(|it| {
                if query.is_empty() {
                    return true;
                }
                format!("{} {} {}", it.kind, it.label, it.sub)
                    .to_lowercase()
                    .contains(&query)
            })
            .take(12)
            .collect();

        div()
            .id("cmd_palette_backdrop")
            .absolute()
            .inset_0()
            .bg(theme.bg_backdrop)
            .flex()
            .items_center()
            .justify_center()
            .p(px(20.0))
            .on_click(cx.listener(|this, _event, _window, cx| {
                this.state.cmd_palette_open = false;
                cx.notify();
            }))
            .child(
                div()
                    .id("cmd_palette_modal")
                    .w_full()
                    .max_w(px(640.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_strong)
                    .rounded(RADIUS_LG)
                    .overflow_hidden()
                    .on_click(|_event, _window, _cx| {
                        // Prevent click on modal from closing it
                    })
                    .child(
                        // Input row
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .p(px(14.0))
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .child(
                                svg()
                                    .path("icons/search.svg")
                                    .w(px(18.0))
                                    .h(px(18.0))
                                    .text_color(theme.text_secondary),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(px(15.0))
                                    .text_color(if query.is_empty() { theme.text_muted } else { theme.text_primary })
                                    .child(if query.is_empty() {
                                        "Type to search 128 conversations, 342 memories, 33 Hive nodes, or commands…".to_string()
                                    } else {
                                        self.state.cmd_query.clone()
                                    }),
                            )
                            .child(
                                div()
                                    .id("cmd_esc_btn")
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(theme.border_strong)
                                    .bg(theme.bg_surface)
                                    .text_color(theme.text_secondary)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.cmd_palette_open = false;
                                        cx.notify();
                                    }))
                                    .child("ESC"),
                            ),
                    )
                    .child(
                        // Results list
                        div()
                            .id("cmd_results_scroll")
                            .max_h(px(380.0))
                            .overflow_y_scroll()
                            .p(px(8.0))
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .children(filtered.into_iter().enumerate().map(|(idx, it)| {
                                let action_code = it.action_code;
                                let target = it.target;
                                div()
                                    .id(SharedString::from(format!("cmd-res-{}", idx)))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .gap(px(12.0))
                                    .px(px(12.0))
                                    .py(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .cursor_pointer()
                                    .bg(if idx == 0 { theme.bg_hover } else { theme.bg_surface })
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.state.cmd_palette_open = false;
                                        match action_code {
                                            0 => {
                                                this.state.add_folder_modal_open = true;
                                            }
                                            1 => {
                                                this.state.toggle_theme();
                                            }
                                            2 => {
                                                this.state.toggle_tech_mode();
                                            }
                                            3 => {
                                                this.state.lock_vault();
                                            }
                                            10 => {
                                                this.state.selected_conversation_title = target.clone();
                                                this.state.navigate(Page::ConversationDetail);
                                            }
                                            20 => {
                                                this.state.selected_memory_id = target.clone();
                                                this.state.navigate(Page::MemoryDetail);
                                            }
                                            30 => {
                                                this.state.select_hive_node(&target, true);
                                                this.state.navigate(Page::Hive);
                                            }
                                            40 => {
                                                this.state.quick_drawer_folder_id = Some(target.clone());
                                                this.state.navigate(Page::Files);
                                            }
                                            _ => {}
                                        }
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.text_primary)
                                                    .child(it.label),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.5))
                                                    .text_color(theme.text_muted)
                                                    .child(it.sub),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .font_family(FONT_MONO)
                                            .text_size(px(10.5))
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .text_color(theme.text_secondary)
                                            .child(it.kind),
                                    )
                            })),
                    ),
            )
    }

    fn render_slide_drawer(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;

        let mem_opt = self
            .state
            .quick_drawer_memory_id
            .clone()
            .and_then(|id| self.state.memories.iter().find(|m| m.id == id).cloned());

        let folder_opt = self.state.quick_drawer_folder_id.clone().and_then(|id| {
            self.state
                .approved_folders
                .iter()
                .find(|f| f.id == id)
                .cloned()
        });

        div()
            .id("quick_drawer_backdrop")
            .absolute()
            .inset_0()
            .bg(theme.bg_backdrop)
            .flex()
            .justify_end()
            .on_click(cx.listener(|this, _event, _window, cx| {
                this.state.quick_drawer_memory_id = None;
                this.state.quick_drawer_folder_id = None;
                cx.notify();
            }))
            .child(
                div()
                    .id("quick_drawer_panel")
                    .w(px(540.0))
                    .h_full()
                    .bg(theme.bg_surface)
                    .border_l_1()
                    .border_color(theme.border_strong)
                    .flex()
                    .flex_col()
                    .on_click(|_event, _window, _cx| {
                        // Prevent closing when clicking drawer content
                    })
                    .child(
                        // Drawer Header
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(20.0))
                            .border_b_1()
                            .border_color(theme.border_subtle)
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
                                            .child(if mem_opt.is_some() {
                                                "PROVENANCE QUICK INSPECTOR"
                                            } else {
                                                "APPROVED DOCUMENT ROOT"
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .mt(px(3.0))
                                            .child(if let Some(m) = &mem_opt {
                                                m.title.to_string()
                                            } else if let Some(f) = &folder_opt {
                                                f.path.clone()
                                            } else {
                                                "Record Inspector".to_string()
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .id("quick_drawer_close_btn")
                                    .w(px(32.0))
                                    .h(px(32.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.quick_drawer_memory_id = None;
                                        this.state.quick_drawer_folder_id = None;
                                        cx.notify();
                                    }))
                                    .child("✕"),
                            ),
                    )
                    .child(
                        // Drawer Body
                        div()
                            .id("quick_drawer_body_scroll")
                            .flex_1()
                            .overflow_y_scroll()
                            .p(px(22.0))
                            .when_some(mem_opt.clone(), |this, m| {
                                this.child(
                                    div()
                                        .flex()
                                        .gap(px(8.0))
                                        .mb(px(14.0))
                                        .child(
                                            div()
                                                .px(px(8.0))
                                                .py(px(3.0))
                                                .rounded(px(5.0))
                                                .bg(theme.bg_inverse)
                                                .text_color(theme.text_inverse)
                                                .text_size(px(11.5))
                                                .font_weight(FontWeight::MEDIUM)
                                                .child(m.mem_type),
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
                                                .child(m.updated),
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
                                                .child(format!("BM25+Vec Score: {}", m.score)),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.text_primary)
                                        .mb(px(16.0))
                                        .child(m.content),
                                )
                                .child(
                                    div()
                                        .p(px(14.0))
                                        .rounded(RADIUS_SM)
                                        .bg(theme.bg_subtle)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .mb(px(16.0))
                                        .child(
                                            div()
                                                .font_family(FONT_MONO)
                                                .text_size(px(11.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.text_muted)
                                                .mb(px(4.0))
                                                .child("WHY THIS MEMORY MATTERS"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.text_secondary)
                                                .child(m.why),
                                        ),
                                )
                                .child(
                                    div()
                                        .font_family(FONT_MONO)
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_muted)
                                        .mb(px(10.0))
                                        .child("CRYPTOGRAPHIC & SOURCE LINEAGE"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .pb(px(8.0))
                                                .border_b_1()
                                                .border_color(theme.border_subtle)
                                                .child(div().text_size(px(11.5)).text_color(theme.text_muted).child("Origin Conversation"))
                                                .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(m.source)),
                                        )
                                        .child(
                                            div()
                                                .pb(px(8.0))
                                                .border_b_1()
                                                .border_color(theme.border_subtle)
                                                .child(div().text_size(px(11.5)).text_color(theme.text_muted).child("State-Core Record ID"))
                                                .child(div().font_family(FONT_MONO).text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(m.id)),
                                        )
                                        .child(
                                            div()
                                                .child(div().text_size(px(11.5)).text_color(theme.text_muted).child("Vault Boundary"))
                                                .child(div().font_family(FONT_MONO).text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child("SQLCipher Page #104 · Verified")),
                                        ),
                                )
                            })
                            .when_some(folder_opt.clone(), |this, f| {
                                this.child(
                                    div()
                                        .flex()
                                        .gap(px(8.0))
                                        .mb(px(14.0))
                                        .child(
                                            div()
                                                .px(px(8.0))
                                                .py(px(3.0))
                                                .rounded(px(5.0))
                                                .bg(theme.bg_inverse)
                                                .text_color(theme.text_inverse)
                                                .text_size(px(11.5))
                                                .font_weight(FontWeight::MEDIUM)
                                                .child(f.status.to_uppercase()),
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
                                                .child(format!("{} Indexed Files · {:.1} MB", f.files_count, f.size_mb)),
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
                                                .child(f.recursive),
                                        ),
                                )
                                .child(
                                    div()
                                        .p(px(12.0))
                                        .bg(theme.bg_subtle)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .rounded(RADIUS_SM)
                                        .mb(px(16.0))
                                        .child(
                                            div()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_size(px(13.0))
                                                .text_color(theme.text_primary)
                                                .mb(px(3.0))
                                                .child("Bounded Extraction Authority"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.5))
                                                .text_color(theme.text_secondary)
                                                .child(format!("Files inside this root are parsed locally via {}. If you revoke this folder, all {} files are immediately fenced from MCP retrieval.", f.dev_provenance, f.files_count)),
                                        ),
                                )
                                .child(
                                    div()
                                        .font_family(FONT_MONO)
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_muted)
                                        .mb(px(8.0))
                                        .child(format!("INDEXED DOCUMENT OBJECTS ({} PREVIEWED)", f.sample_files.len())),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(8.0))
                                        .children(f.sample_files.into_iter().map(|file| {
                                            div()
                                                .p(px(10.0))
                                                .rounded(RADIUS_SM)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .bg(theme.bg_surface)
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .justify_between()
                                                        .child(
                                                            div()
                                                                .font_family(FONT_MONO)
                                                                .text_size(px(12.5))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.text_primary)
                                                                .child(file.name),
                                                        )
                                                        .child(
                                                            div()
                                                                .font_family(FONT_MONO)
                                                                .text_size(px(10.5))
                                                                .px(px(6.0))
                                                                .py(px(1.0))
                                                                .rounded(px(4.0))
                                                                .bg(theme.bg_subtle)
                                                                .border_1()
                                                                .border_color(theme.border_subtle)
                                                                .child(file.status),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .justify_between()
                                                        .mt(px(4.0))
                                                        .text_size(px(11.5))
                                                        .text_color(theme.text_muted)
                                                        .child(format!("Size: {} · Extractor: {}", file.size, file.extractor))
                                                        .child(div().font_family(FONT_MONO).child(file.hash)),
                                                )
                                        })),
                                )
                            }),
                    )
                    .child(
                        // Drawer Footer
                        div()
                            .p(px(20.0))
                            .border_t_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .justify_between()
                            .gap(px(10.0))
                            .when_some(mem_opt, |this, m| {
                                let source_title = m.source.to_string();
                                let mem_id = m.id;
                                this.child(
                                    div()
                                        .id("drawer_open_thread_btn")
                                        .h(px(36.0))
                                        .px(px(14.0))
                                        .rounded(RADIUS_SM)
                                        .border_1()
                                        .border_color(theme.border_strong)
                                        .bg(theme.bg_surface)
                                        .text_color(theme.text_primary)
                                        .text_size(px(12.5))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_click(cx.listener(move |this, _event, _window, cx| {
                                            this.state.quick_drawer_memory_id = None;
                                            this.state.selected_conversation_title = source_title.clone();
                                            this.state.navigate(Page::ConversationDetail);
                                            cx.notify();
                                        }))
                                        .child("Open Source Thread ↗"),
                                )
                                .child(
                                    div()
                                        .id("drawer_full_reader_btn")
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
                                        .on_click(cx.listener(move |this, _event, _window, cx| {
                                            this.state.quick_drawer_memory_id = None;
                                            this.state.selected_memory_id = mem_id.clone();
                                            this.state.navigate(Page::MemoryDetail);
                                            cx.notify();
                                        }))
                                        .child("Full Reader View →"),
                                )
                            })
                            .when_some(folder_opt, |this, f| {
                                let col_name = f.collection;
                                this.child(
                                    div()
                                        .id("drawer_open_col_btn")
                                        .h(px(36.0))
                                        .px(px(14.0))
                                        .rounded(RADIUS_SM)
                                        .border_1()
                                        .border_color(theme.border_strong)
                                        .bg(theme.bg_surface)
                                        .text_color(theme.text_primary)
                                        .text_size(px(12.5))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_click(cx.listener(move |this, _event, _window, cx| {
                                            this.state.quick_drawer_folder_id = None;
                                            this.state.selected_collection_name = col_name.clone();
                                            this.state.navigate(Page::CollectionDetail);
                                            cx.notify();
                                        }))
                                        .child("Open Collection ↗"),
                                )
                                .child(
                                    div()
                                        .id("drawer_close_folder_btn")
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
                                            this.state.quick_drawer_folder_id = None;
                                            cx.notify();
                                        }))
                                        .child("Close Inspector"),
                                )
                            }),
                    ),
            )
    }

    fn render_add_folder_modal(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let path = self.state.add_folder_path.clone();
        let collection = self.state.add_folder_collection.clone();
        let watch_mode = self.state.add_folder_watch_mode.clone();
        let ext_md = self.state.ext_md;
        let ext_pdf = self.state.ext_pdf;
        let ext_docx = self.state.ext_docx;
        let _rule_rec = self.state.rule_recursive;
        let _rule_ignore = self.state.rule_ignore_build;

        div()
            .id("add_folder_backdrop")
            .absolute()
            .inset_0()
            .bg(theme.bg_backdrop)
            .flex()
            .items_center()
            .justify_center()
            .p(px(20.0))
            .on_click(cx.listener(|this, _event, _window, cx| {
                this.state.add_folder_modal_open = false;
                cx.notify();
            }))
            .child(
                div()
                    .id("add_folder_modal")
                    .w_full()
                    .max_w(px(580.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_strong)
                    .rounded(RADIUS_LG)
                    .p(px(24.0))
                    .on_click(|_event, _window, _cx| {
                        // Keep open
                    })
                    .child(
                        // Header
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .child("STATE-CORE::DOCUMENT_ROOTS · EXPLICIT READ GRANT"),
                            )
                            .child(
                                div()
                                    .id("add_folder_close_btn")
                                    .w(px(28.0))
                                    .h(px(28.0))
                                    .rounded(RADIUS_XS)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.add_folder_modal_open = false;
                                        cx.notify();
                                    }))
                                    .child("✕"),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .my(px(6.0))
                            .child("Authorize Local Folder"),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.text_secondary)
                            .mb(px(16.0))
                            .child("Grant Mind Place read-only access to a specific directory. Only selected file formats will be extracted into your local encrypted vault."),
                    )
                    // Path input field
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .mb(px(12.0))
                            .child(div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD).child("Local Directory Path"))
                            .child(
                                div()
                                    .h(px(36.0))
                                    .px(px(12.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_strong)
                                    .bg(theme.bg_surface)
                                    .font_family(FONT_MONO)
                                    .text_size(px(13.0))
                                    .text_color(theme.text_primary)
                                    .flex()
                                    .items_center()
                                    .child(path.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .mt(px(2.0))
                                    .child(div().text_size(px(11.0)).text_color(theme.text_muted).child("Presets:"))
                                    .child(
                                        div()
                                            .id("preset-1-chip")
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .px(px(9.0))
                                            .py(px(3.0))
                                            .rounded(RADIUS_XS)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .text_color(theme.text_secondary)
                                            .cursor_pointer()
                                            .hover(|s| s.border_color(theme.border_focus).text_color(theme.text_primary))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.add_folder_path = "D:\\Projects\\Soramula-Docs".into();
                                                this.state.add_folder_collection = "Engineering".into();
                                                cx.notify();
                                            }))
                                            .child("D:\\Projects\\Soramula-Docs"),
                                    )
                                    .child(
                                        div()
                                            .id("preset-2-chip")
                                            .font_family(FONT_MONO)
                                            .text_size(px(11.0))
                                            .px(px(9.0))
                                            .py(px(3.0))
                                            .rounded(RADIUS_XS)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .bg(theme.bg_subtle)
                                            .text_color(theme.text_secondary)
                                            .cursor_pointer()
                                            .hover(|s| s.border_color(theme.border_focus).text_color(theme.text_primary))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.add_folder_path = "C:\\Users\\kais\\Downloads\\Papers".into();
                                                this.state.add_folder_collection = "Research".into();
                                                cx.notify();
                                            }))
                                            .child("C:\\Users\\kais\\Downloads\\Papers"),
                                    ),
                            ),
                    )
                    // Dropdowns: Target Collection & Watch Mode
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .mb(px(14.0))
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .child(div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD).child("Target Collection"))
                                    .child(
                                        div()
                                            .id("target_collection_input")
                                            .h(px(36.0))
                                            .px(px(12.0))
                                            .rounded(RADIUS_SM)
                                            .border_1()
                                            .border_color(theme.border_strong)
                                            .bg(theme.bg_surface)
                                            .flex()
                                            .items_center()
                                            .text_size(px(13.0))
                                            .child(collection),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .child(div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD).child("Initial Watch State"))
                                    .child(
                                        div()
                                            .id("initial_watch_input")
                                            .h(px(36.0))
                                            .px(px(12.0))
                                            .rounded(RADIUS_SM)
                                            .border_1()
                                            .border_color(theme.border_strong)
                                            .bg(theme.bg_surface)
                                            .flex()
                                            .items_center()
                                            .text_size(px(13.0))
                                            .child(watch_mode),
                                    ),
                            ),
                    )
                    // Extractors
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .mb(px(6.0))
                            .child("Allowed File Extractors (document-core)"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .mb(px(14.0))
                            .child(
                                div()
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .bg(theme.bg_subtle)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).child("Markdown & Plaintext (.md, .txt)"))
                                            .child(div().text_size(px(11.5)).text_color(theme.text_secondary).child("Fast UTF-8 structural chunking with heading preservation.")),
                                    )
                                    .child(div().text_color(theme.text_primary).font_weight(FontWeight::BOLD).child(if ext_md { "✓" } else { " " })),
                            )
                            .child(
                                div()
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .bg(theme.bg_subtle)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).child("PDF Documents (.pdf — bounded extractor)"))
                                            .child(div().text_size(px(11.5)).text_color(theme.text_secondary).child("Executed via document-core/src/pdf.rs with memory bounds.")),
                                    )
                                    .child(div().text_color(theme.text_primary).font_weight(FontWeight::BOLD).child(if ext_pdf { "✓" } else { " " })),
                            )
                            .child(
                                div()
                                    .p(px(10.0))
                                    .rounded(RADIUS_SM)
                                    .bg(theme.bg_subtle)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(12.5)).font_weight(FontWeight::SEMIBOLD).child("Office Documents (.docx — OOXML extractor)"))
                                            .child(div().text_size(px(11.5)).text_color(theme.text_secondary).child("Executed via document-core/src/ooxml.rs.")),
                                    )
                                    .child(div().text_color(theme.text_primary).font_weight(FontWeight::BOLD).child(if ext_docx { "✓" } else { " " })),
                            ),
                    )
                    // Footer
                    .child(
                        div()
                            .pt(px(16.0))
                            .border_t_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .text_color(theme.text_muted)
                                    .child("Revocable anytime via Fence"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .id("cancel-add-folder-btn")
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
                                                this.state.add_folder_modal_open = false;
                                                cx.notify();
                                            }))
                                            .child("Cancel"),
                                    )
                                    .child(
                                        div()
                                            .id("confirm-add-folder-btn")
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
                                                let p = this.state.add_folder_path.clone();
                                                let c = this.state.add_folder_collection.clone();
                                                let w = this.state.add_folder_watch_mode.clone();

                                                let new_folder = ApprovedFolder {
                                                    id: format!("root_{:x}", rand_id()),
                                                    path: p.clone(),
                                                    collection: c,
                                                    files_count: 14,
                                                    size_mb: 11.6,
                                                    formats: vec!["PDF".into(), "DOCX".into(), "MD".into()],
                                                    status: w,
                                                    recursive: "Yes (Recursive)".into(),
                                                    last_synced: "Just now".into(),
                                                    dev_provenance: "root_new · NTFS USN Journal · document-core::pdf,ooxml,md".into(),
                                                    sample_files: vec![
                                                        SampleFile { name: "architecture-overview.pdf".into(), size: "4.8 MB".into(), extractor: "pdf.rs".into(), status: "Bounded · Indexed".into(), hash: "sha256:6b21...90a1".into() },
                                                        SampleFile { name: "implementation-plan.docx".into(), size: "2.3 MB".into(), extractor: "ooxml.rs".into(), status: "Bounded · Indexed".into(), hash: "sha256:8e40...12c9".into() },
                                                        SampleFile { name: "README-notes.md".into(), size: "32 KB".into(), extractor: "utf8".into(), status: "Bounded · Indexed".into(), hash: "sha256:1f09...77b4".into() },
                                                    ],
                                                    scanning: false,
                                                };
                                                this.state.approved_folders.insert(0, new_folder);
                                                this.state.add_folder_modal_open = false;
                                                this.state.set_toast(format!("Authorized and indexed {}", p));
                                                cx.notify();
                                            }))
                                            .child("Authorize & Index Folder →"),
                                    ),
                            ),
                    ),
            )
    }

    fn render_generic_modal(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let modal = self
            .state
            .dialog_modal
            .clone()
            .unwrap_or(crate::models::DialogModal {
                kicker: "MIND PLACE SYSTEM".into(),
                title: "Dialog Title".into(),
                body_html: "Dialog message".into(),
            });

        div()
            .id("dialog_backdrop")
            .absolute()
            .inset_0()
            .bg(theme.bg_backdrop)
            .flex()
            .items_center()
            .justify_center()
            .p(px(20.0))
            .on_click(cx.listener(|this, _event, _window, cx| {
                this.state.dialog_modal = None;
                cx.notify();
            }))
            .child(
                div()
                    .id("dialog_modal")
                    .w_full()
                    .max_w(px(540.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_strong)
                    .rounded(RADIUS_LG)
                    .p(px(24.0))
                    .on_click(|_event, _window, _cx| {
                        // Prevent click on modal from closing it
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .font_family(FONT_MONO)
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .child(modal.kicker),
                            )
                            .child(
                                div()
                                    .id("dialog_close_btn")
                                    .w(px(28.0))
                                    .h(px(28.0))
                                    .rounded(RADIUS_XS)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.state.dialog_modal = None;
                                        cx.notify();
                                    }))
                                    .child("✕"),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .my(px(10.0))
                            .child(modal.title),
                    )
                    .child(
                        div()
                            .text_size(px(13.5))
                            .text_color(theme.text_secondary)
                            .child(modal.body_html),
                    )
                    .child(
                        div().flex().justify_end().mt(px(20.0)).child(
                            div()
                                .id("dialog_ack_btn")
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
                                    this.state.dialog_modal = None;
                                    cx.notify();
                                }))
                                .child("Acknowledge"),
                        ),
                    ),
            )
    }

    fn render_toast_pill(&mut self, msg: String, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;

        div()
            .absolute()
            .bottom(px(24.0))
            .right(px(24.0))
            .px(px(16.0))
            .py(px(10.0))
            .rounded(RADIUS_SM)
            .bg(theme.bg_inverse)
            .text_color(theme.text_inverse)
            .text_size(px(12.5))
            .font_weight(FontWeight::MEDIUM)
            .child(msg)
    }
}

fn rand_id() -> u32 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    now % 0xFFFFF
}
