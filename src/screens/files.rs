use crate::models::{DialogModal, FolderStatusFilter};
use crate::theme::*;
use crate::MindPlaceApp;
use gpui::prelude::*;
use gpui::*;

impl MindPlaceApp {
    pub fn render_files_screen(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme;
        let tech_mode = self.state.tech_mode;
        let total_files = self.state.total_files();
        let total_mb = self.state.total_folder_mb();
        let watching_count = self.state.watching_folder_count();
        let paused_count = self.state.paused_folder_count();
        let active_profile = self.state.encryption_profile.clone();
        let status_filter = self.state.folder_status_filter;
        let query = self.state.folder_search_query.to_lowercase();

        let filtered_folders: Vec<_> = self
            .state
            .approved_folders
            .iter()
            .filter(|f| {
                let match_status = match status_filter {
                    FolderStatusFilter::All => true,
                    FolderStatusFilter::Watching => f.status == "Watching",
                    FolderStatusFilter::Paused => f.status == "Paused",
                };
                let match_q = query.is_empty()
                    || f.path.to_lowercase().contains(&query)
                    || f.collection.to_lowercase().contains(&query)
                    || f.formats
                        .iter()
                        .any(|fmt| fmt.to_lowercase().contains(&query));
                match_status && match_q
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
                                    .child("SECURITY-CORE & DOCUMENT-CORE · ZERO-CRAWL AUTHORITY"),
                            )
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .my(px(4.0))
                                    .child("Files & Secure Vault"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .text_color(theme.text_secondary)
                                    .child("Explicitly authorize which local folders Mind Place is allowed to read, configure bounded extractors, and control SQLCipher vault protection."),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("lock-vault-files-btn")
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
                                        this.state.lock_vault();
                                        cx.notify();
                                    }))
                                    .child("Lock Vault Now"),
                            )
                            .child(
                                div()
                                    .id("verify-fence-files-btn")
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
                                            kicker: "MP-4L1 / MP-9A QUALIFICATION".into(),
                                            title: "Global Recovery Fence & Integrity Check".into(),
                                            body_html: "✓ state-core (schema_v9): 342 records verified\n✓ index-core (schema_v4): FTS5 + Graphify Gen-14 coherent\n✓ archive-core: 128 conversation transcripts checksum OK\n✓ security-core: SQLCipher WAL checkpoint fenced atomically".into(),
                                        });
                                        cx.notify();
                                    }))
                                    .child("Verify Global Fence"),
                            )
                            .child(
                                div()
                                    .id("open-add-folder-top-btn")
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
                                        this.state.add_folder_modal_open = true;
                                        cx.notify();
                                    }))
                                    .child("＋ Add Allowed Folder"),
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
                                    .child("ZERO-CRAWL POLICY ENFORCED"),
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
                                            .child("Mind Place never scans your disk without explicit folder authorization."),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.text_secondary)
                                            .child("Only approved document roots are read via bounded extractors (PDF, DOCX/OOXML, MD/TXT). Removing a folder triggers an immediate Revocation Fence."),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .font_family(FONT_MONO)
                            .text_size(px(11.0))
                            .text_color(theme.text_muted)
                            .child("state-core::document_roots · MP-4L1"),
                    ),
            )
            // 3 Stat Panels
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex_1()
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("AUTHORIZED DOCUMENT ROOTS"))
                            .child(div().text_size(px(24.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).my(px(6.0)).child(format!("{} Folders · {} Files", self.state.approved_folders.len(), total_files)))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).child(format!("{:.1} MB total bounded extraction · {} watching, {} paused.", total_mb, watching_count, paused_count))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("CURATED MEMORY RECORDS"))
                            .child(div().text_size(px(24.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).my(px(6.0)).child("342 Units"))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).child("Linked to source conversations & local files in schema_v9.")),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().font_family(FONT_MONO).text_size(px(11.0)).text_color(theme.text_muted).child("ACTIVE ENCRYPTION PROFILE"))
                            .child(div().text_size(px(24.0)).font_weight(FontWeight::BOLD).text_color(theme.text_primary).my(px(6.0)).child(active_profile))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).child("SQLCipher 4096B pages · PBKDF2-HMAC-SHA512 (256k iter).")),
                    ),
            )
            // Centerpiece: Approved Document Roots Manager
            .child(
                div()
                    .w_full()
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded(RADIUS_MD)
                    .overflow_hidden()
                    .child(
                        // Header
                        div()
                            .p(px(18.0))
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(10.0))
                                            .child(
                                                div()
                                                    .text_size(px(15.5))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.text_primary)
                                                    .child("Approved Document Roots (Read-Authorized Folders)"),
                                            )
                                            .child(
                                                div()
                                                    .font_family(FONT_MONO)
                                                    .text_size(px(11.0))
                                                    .px(px(8.0))
                                                    .py(px(2.0))
                                                    .rounded(px(4.0))
                                                    .bg(theme.bg_inverse)
                                                    .text_color(theme.text_inverse)
                                                    .child(format!("{} Authorized Folders · {} Indexed Files", self.state.approved_folders.len(), total_files)),
                                            )
                                            .when(tech_mode, |this| {
                                                this.child(
                                                    div()
                                                        .font_family(FONT_MONO)
                                                        .text_size(px(11.0))
                                                        .text_color(theme.text_muted)
                                                        .child("document-core::bounded_extractor (max 32MB/obj)"),
                                                )
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.5))
                                            .text_color(theme.text_secondary)
                                            .mt(px(4.0))
                                            .child("Grant or revoke Mind Place read access per directory. Pause live watching at any time or inspect extracted files inside each root."),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .id("rescan-all-folders-btn")
                                            .h(px(32.0))
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
                                                for f in &mut this.state.approved_folders {
                                                    f.last_synced = "Just now".into();
                                                }
                                                this.state.set_toast("All authorized folders verified & synchronized");
                                                cx.notify();
                                            }))
                                            .child("↻ Rescan All"),
                                    )
                                    .child(
                                        div()
                                            .id("add-folder-card-btn")
                                            .h(px(32.0))
                                            .px(px(10.0))
                                            .rounded(RADIUS_SM)
                                            .bg(theme.bg_inverse)
                                            .text_color(theme.text_inverse)
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .cursor_pointer()
                                            .hover(|s| s.opacity(0.9))
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.state.add_folder_modal_open = true;
                                                cx.notify();
                                            }))
                                            .child("＋ Add Allowed Folder"),
                                    ),
                            ),
                    )
                    // Roots Toolbar
                    .child(
                        div()
                            .p(px(12.0))
                            .bg(theme.bg_subtle)
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex_1()
                                    .max_w(px(600.0))
                                    .h(px(34.0))
                                    .px(px(12.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_strong)
                                    .bg(theme.bg_surface)
                                    .flex()
                                    .items_center()
                                    .text_size(px(12.5))
                                    .text_color(theme.text_primary)
                                    .child("Filter authorized folders by path, collection, or file format (PDF, DOCX, MD)…"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(2.0))
                                    .p(px(2.0))
                                    .rounded(RADIUS_SM)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .bg(theme.bg_surface)
                                    .child(self.render_folder_tab_btn("All Roots", FolderStatusFilter::All, cx))
                                    .child(self.render_folder_tab_btn("Watching", FolderStatusFilter::Watching, cx))
                                    .child(self.render_folder_tab_btn("Paused", FolderStatusFilter::Paused, cx)),
                            ),
                    )
                    // Folder List
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(filtered_folders.into_iter().map(|f| {
                                let f_id = f.id.clone();
                                let f_id_inspect = f.id.clone();
                                let f_id_rescan = f.id.clone();
                                let f_id_revoke = f.id.clone();
                                let is_watching = f.status == "Watching";

                                div()
                                    .id(SharedString::from(format!("folder-row-{}", f.id)))
                                    .p(px(16.0))
                                    .border_b_1()
                                    .border_color(theme.border_subtle)
                                    .flex()
                                    .items_start()
                                    .justify_between()
                                    .gap(px(18.0))
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .items_start()
                                            .gap(px(14.0))
                                            .child(
                                                div()
                                                    .w(px(38.0))
                                                    .h(px(38.0))
                                                    .rounded(RADIUS_SM)
                                                    .border_1()
                                                    .border_color(if is_watching { theme.bg_inverse } else { theme.border_strong })
                                                    .bg(if is_watching { theme.bg_inverse } else { theme.bg_subtle })
                                                    .text_color(if is_watching { theme.text_inverse } else { theme.text_primary })
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(15.0))
                                                    .flex_shrink_0()
                                                    .child("▣"),
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
                                                            .gap(px(8.0))
                                                            .child(
                                                                div()
                                                                    .font_family(FONT_MONO)
                                                                    .text_size(px(13.0))
                                                                    .font_weight(FontWeight::BOLD)
                                                                    .text_color(theme.text_primary)
                                                                    .child(f.path.clone()),
                                                            )
                                                            .child(
                                                                div()
                                                                    .px(px(8.0))
                                                                    .py(px(2.0))
                                                                    .rounded(px(5.0))
                                                                    .bg(theme.bg_inverse)
                                                                    .text_color(theme.text_inverse)
                                                                    .text_size(px(11.0))
                                                                    .font_weight(FontWeight::MEDIUM)
                                                                    .child(f.collection.clone()),
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .gap(px(4.0))
                                                                    .children(f.formats.iter().map(|fmt| {
                                                                        div()
                                                                            .font_family(FONT_MONO)
                                                                            .px(px(6.0))
                                                                            .py(px(1.0))
                                                                            .rounded(px(4.0))
                                                                            .border_1()
                                                                            .border_color(theme.border_subtle)
                                                                            .bg(theme.bg_subtle)
                                                                            .text_size(px(11.0))
                                                                            .text_color(theme.text_secondary)
                                                                            .child(fmt.clone())
                                                                    })),
                                                            ),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(8.0))
                                                            .mt(px(4.0))
                                                            .text_size(px(12.0))
                                                            .text_color(theme.text_secondary)
                                                            .child(format!("{} files ({:.1} MB)", f.files_count, f.size_mb))
                                                            .child("·")
                                                            .child(f.recursive.clone())
                                                            .child("·")
                                                            .child(format!("Synced {}", f.last_synced)),
                                                    )
                                                    .when(tech_mode, |this| {
                                                        this.child(
                                                            div()
                                                                .p(px(6.0))
                                                                .rounded(RADIUS_XS)
                                                                .bg(theme.bg_subtle)
                                                                .border_1()
                                                                .border_color(theme.border_subtle)
                                                                .font_family(FONT_MONO)
                                                                .text_size(px(11.0))
                                                                .text_color(theme.text_secondary)
                                                                .mt(px(4.0))
                                                                .child(f.dev_provenance.clone()),
                                                        )
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(6.0))
                                            .child(
                                                // Watch / Pause Pill
                                                div()
                                                    .id(SharedString::from(format!("toggle-watch-{}", f.id)))
                                                    .h(px(30.0))
                                                    .px(px(10.0))
                                                    .rounded(RADIUS_SM)
                                                    .border_1()
                                                    .border_color(if is_watching { theme.bg_inverse } else { theme.border_strong })
                                                    .bg(if is_watching { theme.bg_inverse } else { theme.bg_surface })
                                                    .text_color(if is_watching { theme.text_inverse } else { theme.text_secondary })
                                                    .font_family(FONT_MONO)
                                                    .text_size(px(11.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(6.0))
                                                    .cursor_pointer()
                                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                                        if let Some(folder) = this.state.approved_folders.iter_mut().find(|x| x.id == f_id) {
                                                            folder.status = if folder.status == "Watching" {
                                                                "Paused".into()
                                                            } else {
                                                                "Watching".into()
                                                            };
                                                            let path = folder.path.clone();
                                                            let stat = folder.status.clone();
                                                            this.state.set_toast(format!("{} is now {}", path, stat));
                                                        }
                                                        cx.notify();
                                                    }))
                                                    .child(
                                                        div()
                                                            .w(px(6.0))
                                                            .h(px(6.0))
                                                            .rounded_full()
                                                            .bg(if is_watching { theme.text_inverse } else { theme.text_secondary }),
                                                    )
                                                    .child(f.status.clone()),
                                            )
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!("inspect-f-{}", f.id)))
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
                                                        this.state.quick_drawer_folder_id = Some(f_id_inspect.clone());
                                                        cx.notify();
                                                    }))
                                                    .child(format!("Inspect Files ({})", f.files_count)),
                                            )
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!("rescan-f-{}", f.id)))
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
                                                        if let Some(folder) = this.state.approved_folders.iter_mut().find(|x| x.id == f_id_rescan) {
                                                            folder.last_synced = "Just now".into();
                                                            let path = folder.path.clone();
                                                            let count = folder.files_count;
                                                            this.state.set_toast(format!("Synced {} files in {}", count, path));
                                                        }
                                                        cx.notify();
                                                    }))
                                                    .child("↻ Rescan"),
                                            )
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!("fence-f-{}", f.id)))
                                                    .h(px(30.0))
                                                    .px(px(10.0))
                                                    .rounded(RADIUS_SM)
                                                    .border_1()
                                                    .border_color(theme.border_strong)
                                                    .bg(theme.bg_surface)
                                                    .text_color(theme.text_secondary)
                                                    .text_size(px(12.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(theme.bg_inverse).text_color(theme.text_inverse))
                                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                                        if let Some(pos) = this.state.approved_folders.iter().position(|x| x.id == f_id_revoke) {
                                                            let removed = this.state.approved_folders.remove(pos);
                                                            this.state.set_toast(format!("Revocation Fence enforced: {} removed & {} files fenced", removed.path, removed.files_count));
                                                        }
                                                        cx.notify();
                                                    }))
                                                    .child("✕ Fence"),
                                            ),
                                    )
                            })),
                    ),
            )
            // Two-column bottom panels
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    // Panel 1: Vault Protection Boundaries
                    .child(
                        div()
                            .flex_1()
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).child("Vault Protection Boundaries"))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mb(px(16.0)).child("Toggle encrypted storage enforcement across local subsystems."))
                            .child(self.render_boundary_row("Conversation Archive (archive-core)", "Original multi-provider transcripts and raw DOM snapshots.", "archive", cx))
                            .child(self.render_boundary_row("Curated Memories (state-core)", "Derived insights, authority rankings, and provenance links.", "memories", cx))
                            .child(self.render_boundary_row("Extracted Document Objects (document-core)", "Parsed PDF and Office Open XML chunks from approved folders.", "files", cx))
                            .child(self.render_boundary_row("Derived Search & Graph Indexes (index-core)", "FTS5 lexical tables and Graphify generation snapshots.", "indexes", cx)),
                    )
                    // Panel 2: Cryptographic & Lock Policy
                    .child(
                        div()
                            .w(px(460.0))
                            .p(px(20.0))
                            .rounded(RADIUS_MD)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.bg_surface)
                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).child("Cryptographic & Lock Policy"))
                            .child(div().text_size(px(12.5)).text_color(theme.text_secondary).mb(px(16.0)).child("Configure runtime decryption caching and automatic vault lock behavior."))
                            .child(div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD).mb(px(6.0)).child("Runtime Encryption Profile"))
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
                                    .mb(px(14.0))
                                    .child("Balanced — Secure page cache, 4.2ms average read"),
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
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Immediate Revocation Fence on Folder Removal"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Purge derived chunks and block MCP reads the instant a folder is revoked.")),
                                    )
                                    .child(self.render_switch(true)),
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
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).child("Auto-Lock Vault on Window Close"))
                                            .child(div().text_size(px(12.0)).text_color(theme.text_secondary).child("Zeroize in-memory master key when window closes.")),
                                    )
                                    .child(self.render_switch(true)),
                            )
                            .child(
                                div()
                                    .id("inspect-crypto-manifest-btn")
                                    .w_full()
                                    .h(px(36.0))
                                    .mt(px(14.0))
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
                                            kicker: "SECURITY-CORE MANIFEST".into(),
                                            title: "Cryptographic Vault Manifest".into(),
                                            body_html: "Mind Place encrypts your primary SQLite state database, cold archive blobs, and derived indexes using SQLCipher AES-256-CBC/GCM primitives.\n\nCipher: AES-256 · KDF: PBKDF2-HMAC-SHA512\nPage Size: 4096 bytes · HMAC Check: SHA512\nRecovery Boundary: Atomic fail-closed fence".into(),
                                        });
                                        cx.notify();
                                    }))
                                    .child("Inspect Cryptographic Manifest →"),
                            ),
                    ),
            )
    }

    fn render_boundary_row(
        &mut self,
        label: &'static str,
        desc: &'static str,
        key: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_on = *self.state.vault_boundaries.get(key).unwrap_or(&true);

        div()
            .py(px(12.0))
            .border_b_1()
            .border_color(theme.border_subtle)
            .flex()
            .items_center()
            .justify_between()
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_primary)
                            .child(label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.text_secondary)
                            .child(desc),
                    ),
            )
            .child(
                div()
                    .id(SharedString::from(format!("switch-boundary-{}", key)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _event, _window, cx| {
                        let cur = *this.state.vault_boundaries.get(key).unwrap_or(&true);
                        this.state.vault_boundaries.insert(key.to_string(), !cur);
                        this.state.set_toast("Capability boundary updated");
                        cx.notify();
                    }))
                    .child(self.render_switch(is_on)),
            )
    }

    pub fn render_switch(&self, on: bool) -> impl IntoElement {
        let theme = self.state.theme;

        div()
            .w(px(38.0))
            .h(px(21.0))
            .rounded_full()
            .bg(if on { theme.bg_inverse } else { theme.bg_muted })
            .border_1()
            .border_color(if on {
                theme.bg_inverse
            } else {
                theme.border_strong
            })
            .p(px(2.0))
            .flex()
            .items_center()
            .justify_start()
            .child(
                div()
                    .w(px(15.0))
                    .h(px(15.0))
                    .rounded_full()
                    .bg(if on {
                        theme.text_inverse
                    } else {
                        theme.bg_surface
                    })
                    .when(on, |this| this.ml(px(17.0))),
            )
    }

    fn render_folder_tab_btn(
        &mut self,
        label: &'static str,
        filter: FolderStatusFilter,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.theme;
        let is_active = self.state.folder_status_filter == filter;

        div()
            .id(SharedString::from(format!("folder-filter-{:?}", filter)))
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
                this.state.folder_status_filter = filter;
                cx.notify();
            }))
            .child(label)
    }
}
