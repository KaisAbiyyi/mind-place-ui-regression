use crate::hive_data::{get_hive_edges, get_hive_nodes, HiveNode};
use crate::models::*;
use crate::theme::{DensityMode, Theme, ThemeMode};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct BenchmarkRunner {
    pub scenario: String,
    pub duration_sec: f64,
    pub start_time: Instant,
    pub last_frame_time: Instant,
    pub frame_times: Vec<f64>,
    pub dataset_load_time_ms: f64,
    pub output_file: Option<String>,
}

pub struct AppState {
    pub theme: Theme,
    pub density: DensityMode,
    pub tech_mode: bool,
    pub sidebar_collapsed: bool,
    pub active_page: Page,
    pub previous_page: Page,

    // Overlays
    pub vault_locked: bool,
    pub cmd_palette_open: bool,
    pub cmd_query: String,
    pub quick_drawer_memory_id: Option<String>,
    pub quick_drawer_folder_id: Option<String>,
    pub dialog_modal: Option<DialogModal>,
    pub add_folder_modal_open: bool,
    pub acl_modal_provider: Option<String>,
    pub toast: Option<(String, Instant)>,

    // Add folder modal form inputs
    pub add_folder_path: String,
    pub add_folder_collection: String,
    pub add_folder_watch_mode: String,
    pub ext_md: bool,
    pub ext_pdf: bool,
    pub ext_docx: bool,
    pub rule_recursive: bool,
    pub rule_ignore_build: bool,

    // Datasets
    pub conversations: Vec<Conversation>,
    pub starred_conversations: HashSet<String>,
    pub conv_search_query: String,
    pub conv_provider_filter: String,
    pub conv_tab_filter: ConvTabFilter,

    pub collections: Vec<MemoryCollection>,
    pub memories: Vec<CuratedMemory>,
    pub memory_search_query: String,
    pub memory_view_tab: MemoryViewTab,

    pub approved_folders: Vec<ApprovedFolder>,
    pub folder_search_query: String,
    pub folder_status_filter: FolderStatusFilter,

    pub connected_providers: HashMap<String, bool>,
    pub provider_permissions: HashMap<String, (bool, bool)>, // (read, write)

    pub setup_step: usize,
    pub encryption_profile: String,
    pub vault_boundaries: HashMap<String, bool>,

    // Active detail page targets
    pub selected_conversation_title: String,
    pub selected_collection_name: String,
    pub selected_memory_id: String,

    // Hive Mind Graph state
    pub hive_nodes: Vec<HiveNode>,
    pub hive_edges: Vec<(String, String)>,
    pub hive_selected_node_id: String,
    pub hive_hovered_node_id: Option<String>,
    pub hive_filter: String,
    pub hive_search_query: String,
    pub hive_trace_active: bool,
    pub hive_camera_x: f32,
    pub hive_camera_y: f32,
    pub hive_camera_zoom: f32,
    pub hive_target_x: f32,
    pub hive_target_y: f32,
    pub hive_target_zoom: f32,
    pub hive_is_dragging: bool,
    pub hive_drag_start: Option<(f32, f32)>,
    pub hive_anim_time: f32,
    pub hive_benchmark: Option<BenchmarkRunner>,
}

impl AppState {
    pub fn new() -> Self {
        let mut starred = HashSet::new();
        starred.insert("BOMC Builder — export architecture".to_string());
        starred.insert("Building the Mind Place archive".to_string());

        let mut providers = HashMap::new();
        providers.insert("ChatGPT".to_string(), true);
        providers.insert("Claude".to_string(), false);
        providers.insert("Gemini".to_string(), false);

        let mut permissions = HashMap::new();
        permissions.insert("ChatGPT".to_string(), (true, false));
        permissions.insert("Claude".to_string(), (false, false));
        permissions.insert("Gemini".to_string(), (false, false));

        let mut vault_boundaries = HashMap::new();
        vault_boundaries.insert("archive".to_string(), true);
        vault_boundaries.insert("memories".to_string(), true);
        vault_boundaries.insert("files".to_string(), true);
        vault_boundaries.insert("indexes".to_string(), true);

        let conversations = vec![
            Conversation {
                title: "Building the Mind Place archive",
                provider: "ChatGPT",
                collection: "Product",
                timestamp: "Today · 21:42",
                sha: "sha256:4a91...c802",
                msgs: 14,
            },
            Conversation {
                title: "BOMC Builder — export architecture",
                provider: "Claude",
                collection: "Engineering",
                timestamp: "Today · 18:16",
                sha: "sha256:8f20...19b4",
                msgs: 22,
            },
            Conversation {
                title: "Research: semantic retrieval",
                provider: "ChatGPT",
                collection: "Research",
                timestamp: "Today · 14:08",
                sha: "sha256:3c71...90e1",
                msgs: 9,
            },
            Conversation {
                title: "Piano sheet-to-key mapping",
                provider: "ChatGPT",
                collection: "Learning",
                timestamp: "Yesterday · 22:30",
                sha: "sha256:7d12...44a9",
                msgs: 18,
            },
            Conversation {
                title: "V60 brewing ratios",
                provider: "Gemini",
                collection: "Life & goals",
                timestamp: "Yesterday · 11:20",
                sha: "sha256:1b08...7f3c",
                msgs: 7,
            },
            Conversation {
                title: "Hive graph product direction",
                provider: "ChatGPT",
                collection: "Product",
                timestamp: "Oct 31 · 20:14",
                sha: "sha256:6e55...2d10",
                msgs: 16,
            },
            Conversation {
                title: "Encrypted vault architecture",
                provider: "Claude",
                collection: "Engineering",
                timestamp: "Oct 30 · 16:02",
                sha: "sha256:9a44...01cc",
                msgs: 31,
            },
            Conversation {
                title: "Provider MCP connectivity",
                provider: "ChatGPT",
                collection: "Research",
                timestamp: "Oct 29 · 19:44",
                sha: "sha256:2f99...e408",
                msgs: 12,
            },
        ];

        let collections = vec![
            MemoryCollection {
                name: "Engineering",
                count: 142,
                desc: "Projects, architecture, implementation notes and technical decisions.",
                pct: 82,
            },
            MemoryCollection {
                name: "Research",
                count: 87,
                desc: "Questions, sources, investigations and evolving hypotheses.",
                pct: 61,
            },
            MemoryCollection {
                name: "Learning",
                count: 56,
                desc: "Music, programming, practice notes and reference material.",
                pct: 44,
            },
            MemoryCollection {
                name: "Life & goals",
                count: 34,
                desc: "Personal plans, preferences and recurring themes.",
                pct: 31,
            },
            MemoryCollection {
                name: "Product ideas",
                count: 18,
                desc: "Unfinished concepts, product direction and design exploration.",
                pct: 25,
            },
            MemoryCollection {
                name: "Unsorted",
                count: 5,
                desc: "Newly captured memories awaiting organization.",
                pct: 12,
            },
        ];

        let memories = vec![
            // Engineering
            CuratedMemory {
                id: "Engineering-0".into(),
                collection: "Engineering",
                title: "BOMC Exact must preserve fidelity while reducing export size",
                content: "The optimization target is not merely a smaller file. The Exact export must preserve canonical geometry, material assignment, and the accepted visual result while reducing package size relative to the source SKP.",
                source: "BOMC Builder — export architecture",
                mem_type: "Derived insight",
                updated: "Oct 27, 2026",
                why: "A decision constraint that keeps optimization measurable without allowing fidelity regressions.",
                score: "0.98",
            },
            CuratedMemory {
                id: "Engineering-1".into(),
                collection: "Engineering",
                title: "Sibambo Rooms groups physical openings as assemblies",
                content: "A window or door should be represented as one physical opening assembly. Individual subfaces remain available for measurement, but the visual selection should highlight the complete opening.",
                source: "BOMC Builder — export architecture",
                mem_type: "Project requirement",
                updated: "Oct 27, 2026",
                why: "This requirement prevents panel-level detection from fragmenting one real-world opening.",
                score: "0.94",
            },
            CuratedMemory {
                id: "Engineering-2".into(),
                collection: "Engineering",
                title: "Mind Place vault recovery must be globally fenced",
                content: "Backup and restore must coordinate database, WAL, indexes, archive, and objects under one recovery boundary. Partial publication must fail closed and remain recoverable.",
                source: "Encrypted vault architecture",
                mem_type: "Architecture decision",
                updated: "Oct 25, 2026",
                why: "The recovery contract protects consistency across all vault data, not only the primary database.",
                score: "0.99",
            },
            CuratedMemory {
                id: "Engineering-3".into(),
                collection: "Engineering",
                title: "Cloud-first qualification excludes owner-only gates",
                content: "Cloud CI can qualify repository behavior, but physical Windows and owner-environment gates remain NOT_RUN unless explicitly authorized and executed.",
                source: "Building the Mind Place archive",
                mem_type: "Operating rule",
                updated: "Oct 31, 2026",
                why: "Separating cloud evidence from owner-machine evidence prevents unsupported acceptance claims.",
                score: "0.92",
            },
            CuratedMemory {
                id: "Engineering-4".into(),
                collection: "Engineering",
                title: "Self-review requires exact HEAD evidence",
                content: "Implementors must review the exact commit SHA, tests, security implications, and CI evidence before a task is marked ready for merge.",
                source: "Building the Mind Place archive",
                mem_type: "Process rule",
                updated: "Oct 27, 2026",
                why: "A reproducible evidence trail is required for reliable handoff and merge decisions.",
                score: "0.95",
            },
            // Research
            CuratedMemory {
                id: "Research-0".into(),
                collection: "Research",
                title: "Research 01 combines quantitative and qualitative evaluation",
                content: "The BOMC comparison uses measurable export properties alongside an independent architect's visual assessment of Exact and visual output. The research design is therefore mixed-method rather than purely quantitative.",
                source: "BOMC Builder — export architecture",
                mem_type: "Research method",
                updated: "Nov 2, 2026",
                why: "The architect review adds an independent professional perspective that file metrics cannot capture.",
                score: "0.96",
            },
            CuratedMemory {
                id: "Research-1".into(),
                collection: "Research",
                title: "Visual fidelity needs an independent architecture review",
                content: "An architect should inspect representative model outputs and record visible differences, artifacts, readability, and practical suitability without being guided toward a preferred result.",
                source: "BOMC Builder — export architecture",
                mem_type: "Evaluation protocol",
                updated: "Nov 2, 2026",
                why: "Independent observation reduces the risk of treating technical similarity as equivalent to professional visual acceptability.",
                score: "0.93",
            },
            CuratedMemory {
                id: "Research-2".into(),
                collection: "Research",
                title: "Retrieval should preserve source traceability",
                content: "A derived answer should retain references to the original conversation, message, provider, timestamp, and extraction path.",
                source: "Research: semantic retrieval",
                mem_type: "Design principle",
                updated: "Oct 29, 2026",
                why: "Traceability makes memory useful without hiding the provenance of the underlying information.",
                score: "0.97",
            },
            CuratedMemory {
                id: "Research-3".into(),
                collection: "Research",
                title: "Agent permissions should be scoped per provider",
                content: "Read and write permissions are separate capabilities. A provider should receive only explicitly granted collections and operations.",
                source: "Provider MCP connectivity",
                mem_type: "Security principle",
                updated: "Oct 28, 2026",
                why: "Narrow permissions reduce accidental exposure and make revocation easier to reason about.",
                score: "0.96",
            },
            // Learning
            CuratedMemory {
                id: "Learning-0".into(),
                collection: "Learning",
                title: "Piano sheet reading maps pitch, duration, and simultaneity",
                content: "A piano score communicates pitch by vertical staff position, duration by note shape and rhythmic notation, and simultaneous notes through vertical alignment. Chord grouping is not the same as lyric grouping.",
                source: "Piano sheet-to-key mapping",
                mem_type: "Learning note",
                updated: "Nov 1, 2026",
                why: "This distinction helps translate a guitar-oriented source into piano parts without inventing note groupings.",
                score: "0.91",
            },
            CuratedMemory {
                id: "Learning-1".into(),
                collection: "Learning",
                title: "An 88-key piano spans A0 through C8",
                content: "A standard 88-key piano begins at A0 and ends at C8. Octave labels should follow the chosen convention consistently throughout any note-conversion interface.",
                source: "Piano sheet-to-key mapping",
                mem_type: "Reference",
                updated: "Nov 1, 2026",
                why: "The range check prevents generated note labels from drifting by an octave.",
                score: "0.90",
            },
            CuratedMemory {
                id: "Learning-2".into(),
                collection: "Learning",
                title: "Left-hand bass should follow the chord root or inversion",
                content: "For a chord inversion, the bass note can reflect the inversion when the arrangement calls for it; otherwise the root remains a stable foundation. The score determines the intended voicing.",
                source: "Piano sheet-to-key mapping",
                mem_type: "Practice note",
                updated: "Oct 31, 2026",
                why: "This keeps bass choices musical rather than applying a rigid one-note-per-chord rule.",
                score: "0.89",
            },
            // Life & goals
            CuratedMemory {
                id: "Life & goals-0".into(),
                collection: "Life & goals",
                title: "V60 15 g is a useful baseline recipe",
                content: "A practical starting point is 15 g coffee with around 225–240 g water, then adjust grind, pouring, and total brew time based on taste and the beans.",
                source: "V60 brewing ratios",
                mem_type: "Personal reference",
                updated: "Nov 2, 2026",
                why: "A repeatable baseline makes it easier to identify which brewing variable changed the cup.",
                score: "0.94",
            },
            CuratedMemory {
                id: "Life & goals-1".into(),
                collection: "Life & goals",
                title: "Iced V60 dilution should be planned as part of the ratio",
                content: "When brewing directly over ice, include the ice mass in the total beverage water budget rather than treating it as an afterthought.",
                source: "V60 brewing ratios",
                mem_type: "Personal reference",
                updated: "Oct 31, 2026",
                why: "Accounting for meltwater prevents an iced recipe from becoming unintentionally weak.",
                score: "0.91",
            },
            CuratedMemory {
                id: "Life & goals-2".into(),
                collection: "Life & goals",
                title: "Piano practice is organized around chord shapes and lyrics",
                content: "The preferred learning layout keeps lyrics readable while distinguishing right-hand keys, left-hand chords, and bass notes. Vertical stacking is reserved for notes played simultaneously.",
                source: "Piano sheet-to-key mapping",
                mem_type: "Preference",
                updated: "Oct 31, 2026",
                why: "This interface preference directly affects how sheet-derived arrangements should be presented.",
                score: "0.88",
            },
            // Product ideas
            CuratedMemory {
                id: "Product ideas-0".into(),
                collection: "Product ideas",
                title: "Hive Mind should reveal relationships without forcing them",
                content: "The graph should make dense local connections discoverable while keeping uncertain or isolated memories visible. A relationship should be explainable and traceable to source evidence.",
                source: "Hive graph product direction",
                mem_type: "Product principle",
                updated: "Oct 31, 2026",
                why: "Showing uncertainty is more trustworthy than drawing attractive but unsupported links.",
                score: "0.98",
            },
            CuratedMemory {
                id: "Product ideas-1".into(),
                collection: "Product ideas",
                title: "Mind Place should feel local-first and provider-neutral",
                content: "The vault is the source of truth. AI providers are optional integrations with distinct permissions, not the owners of the user's memory.",
                source: "Building the Mind Place archive",
                mem_type: "Product principle",
                updated: "Oct 28, 2026",
                why: "This separates durable user knowledge from the lifecycle of any one AI provider.",
                score: "0.99",
            },
            // Unsorted
            CuratedMemory {
                id: "Unsorted-0".into(),
                collection: "Unsorted",
                title: "An imported note needs source attribution",
                content: "Imported snippets should retain their original source and import time until they can be reviewed and organized.",
                source: "Building the Mind Place archive",
                mem_type: "Captured note",
                updated: "Oct 26, 2026",
                why: "Attribution makes later review possible even before categorization.",
                score: "0.82",
            },
        ];

        let approved_folders = vec![
            ApprovedFolder {
                id: "root_01h9x".into(),
                path: "C:\\Users\\kais\\Documents\\Engineering-Specs".into(),
                collection: "Engineering".into(),
                files_count: 28,
                size_mb: 18.4,
                formats: vec!["PDF".into(), "DOCX".into(), "MD".into()],
                status: "Watching".into(),
                recursive: "Yes (Recursive)".into(),
                last_synced: "2 mins ago".into(),
                dev_provenance: "root_01h9x · NTFS USN Journal #884120 · document-core::ooxml,pdf"
                    .into(),
                sample_files: vec![
                    SampleFile {
                        name: "bomc-exact-export-spec.pdf".into(),
                        size: "4.2 MB".into(),
                        extractor: "pdf.rs".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:7c91...e102".into(),
                    },
                    SampleFile {
                        name: "vault-recovery-fencing.docx".into(),
                        size: "1.8 MB".into(),
                        extractor: "ooxml.rs".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:3a44...89b1".into(),
                    },
                    SampleFile {
                        name: "sibambo-opening-assemblies.pdf".into(),
                        size: "6.1 MB".into(),
                        extractor: "pdf.rs".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:9f02...44c8".into(),
                    },
                    SampleFile {
                        name: "ntfs-usn-authority-notes.md".into(),
                        size: "48 KB".into(),
                        extractor: "utf8".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:1d88...20a4".into(),
                    },
                ],
                scanning: false,
            },
            ApprovedFolder {
                id: "root_01h9y".into(),
                path: "D:\\Workspaces\\Research-Corpus".into(),
                collection: "Research".into(),
                files_count: 19,
                size_mb: 31.0,
                formats: vec!["PDF".into(), "MD".into()],
                status: "Watching".into(),
                recursive: "Yes (Recursive)".into(),
                last_synced: "14 mins ago".into(),
                dev_provenance: "root_01h9y · NTFS USN Journal #884095 · document-core::pdf".into(),
                sample_files: vec![
                    SampleFile {
                        name: "architect-visual-fidelity-protocol.pdf".into(),
                        size: "8.7 MB".into(),
                        extractor: "pdf.rs".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:5e12...77d0".into(),
                    },
                    SampleFile {
                        name: "hybrid-bm25-vector-bakeoff.pdf".into(),
                        size: "5.4 MB".into(),
                        extractor: "pdf.rs".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:2b67...91f3".into(),
                    },
                    SampleFile {
                        name: "semantic-retrieval-notes.md".into(),
                        size: "112 KB".into(),
                        extractor: "utf8".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:8c30...11e9".into(),
                    },
                ],
                scanning: false,
            },
            ApprovedFolder {
                id: "root_01h9z".into(),
                path: "C:\\Users\\kais\\Notes\\Personal-Knowledge".into(),
                collection: "Learning".into(),
                files_count: 9,
                size_mb: 2.1,
                formats: vec!["MD".into(), "TXT".into()],
                status: "Paused".into(),
                recursive: "No (Top-level)".into(),
                last_synced: "Yesterday · 22:15".into(),
                dev_provenance: "root_01h9z · Poll Watcher · state-core::document_scanner".into(),
                sample_files: vec![
                    SampleFile {
                        name: "piano-88key-voicing-reference.md".into(),
                        size: "64 KB".into(),
                        extractor: "utf8".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:4d90...66a2".into(),
                    },
                    SampleFile {
                        name: "v60-iced-brew-ratios.txt".into(),
                        size: "18 KB".into(),
                        extractor: "utf8".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:6a11...09c7".into(),
                    },
                    SampleFile {
                        name: "chord-inversion-bass-rules.md".into(),
                        size: "39 KB".into(),
                        extractor: "utf8".into(),
                        status: "Bounded · Indexed".into(),
                        hash: "sha256:0f55...32b8".into(),
                    },
                ],
                scanning: false,
            },
        ];

        let hive_nodes = get_hive_nodes();

        Self {
            theme: Theme::light(),
            density: DensityMode::Comfortable,
            tech_mode: false,
            sidebar_collapsed: false,
            active_page: Page::Home,
            previous_page: Page::Home,

            vault_locked: false,
            cmd_palette_open: false,
            cmd_query: String::new(),
            quick_drawer_memory_id: None,
            quick_drawer_folder_id: None,
            dialog_modal: None,
            add_folder_modal_open: false,
            acl_modal_provider: None,
            toast: None,

            add_folder_path: "D:\\Projects\\Soramula-Docs".to_string(),
            add_folder_collection: "Engineering".to_string(),
            add_folder_watch_mode: "Watching".to_string(),
            ext_md: true,
            ext_pdf: true,
            ext_docx: true,
            rule_recursive: true,
            rule_ignore_build: true,

            conversations,
            starred_conversations: starred,
            conv_search_query: String::new(),
            conv_provider_filter: "all".to_string(),
            conv_tab_filter: ConvTabFilter::All,

            collections,
            memories,
            memory_search_query: String::new(),
            memory_view_tab: MemoryViewTab::Collections,

            approved_folders,
            folder_search_query: String::new(),
            folder_status_filter: FolderStatusFilter::All,

            connected_providers: providers,
            provider_permissions: permissions,

            setup_step: 0,
            encryption_profile: "Balanced".to_string(),
            vault_boundaries,

            selected_conversation_title: "Building the Mind Place archive".to_string(),
            selected_collection_name: "Engineering".to_string(),
            selected_memory_id: "Engineering-0".to_string(),

            hive_nodes,
            hive_edges: get_hive_edges(),
            hive_selected_node_id: "mind".to_string(),
            hive_hovered_node_id: None,
            hive_filter: "all".to_string(),
            hive_search_query: String::new(),
            hive_trace_active: true,
            hive_camera_x: 600.0,
            hive_camera_y: 400.0,
            hive_camera_zoom: 1.0,
            hive_target_x: 600.0,
            hive_target_y: 400.0,
            hive_target_zoom: 1.0,
            hive_is_dragging: false,
            hive_drag_start: None,
            hive_anim_time: 0.0,
            hive_benchmark: None,
        }
    }

    pub fn load_hive_dataset(&mut self, json_str: &str) -> Result<(usize, usize), String> {
        #[derive(serde::Deserialize)]
        struct RawDataset {
            nodes: Vec<HiveNode>,
            edges: Vec<[String; 2]>,
        }
        let ds: RawDataset = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
        let node_count = ds.nodes.len();
        let edge_count = ds.edges.len();
        self.hive_nodes = ds.nodes;
        self.hive_edges = ds.edges.into_iter().map(|[a, b]| (a, b)).collect();
        if let Some(first) = self.hive_nodes.first() {
            self.hive_selected_node_id = first.id.clone();
        }
        self.hive_camera_x = 600.0;
        self.hive_camera_y = 400.0;
        self.hive_target_x = 600.0;
        self.hive_target_y = 400.0;
        self.hive_camera_zoom = 1.0;
        self.hive_target_zoom = 1.0;
        Ok((node_count, edge_count))
    }

    pub fn set_toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), Instant::now()));
    }

    pub fn navigate(&mut self, page: Page) {
        self.previous_page = self.active_page;
        self.active_page = page;
    }

    pub fn toggle_theme(&mut self) {
        if self.theme.mode == ThemeMode::Light {
            self.theme = Theme::dark();
            self.set_toast("High-Contrast Dark Mode enabled");
        } else {
            self.theme = Theme::light();
            self.set_toast("Pure Light Mode enabled");
        }
    }

    pub fn toggle_tech_mode(&mut self) {
        self.tech_mode = !self.tech_mode;
        if self.tech_mode {
            self.set_toast("Technical Provenance Mode ON (SHA-256 & BM25 visible)");
        } else {
            self.set_toast("Technical Provenance Mode OFF");
        }
    }

    pub fn toggle_density(&mut self) {
        if self.density == DensityMode::Comfortable {
            self.density = DensityMode::Compact;
            self.set_toast("Compact density enabled");
        } else {
            self.density = DensityMode::Comfortable;
            self.set_toast("Comfortable density enabled");
        }
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
    }

    pub fn lock_vault(&mut self) {
        self.vault_locked = true;
        self.set_toast("Vault locked — in-memory keys zeroized");
    }

    pub fn unlock_vault(&mut self) {
        self.vault_locked = false;
        self.set_toast("Vault unlocked · SQLCipher integrity verified");
    }

    pub fn total_files(&self) -> usize {
        self.approved_folders.iter().map(|f| f.files_count).sum()
    }

    pub fn total_folder_mb(&self) -> f64 {
        self.approved_folders.iter().map(|f| f.size_mb).sum()
    }

    pub fn watching_folder_count(&self) -> usize {
        self.approved_folders
            .iter()
            .filter(|f| f.status == "Watching")
            .count()
    }

    pub fn paused_folder_count(&self) -> usize {
        self.approved_folders.len() - self.watching_folder_count()
    }

    pub fn connected_provider_count(&self) -> usize {
        self.connected_providers.values().filter(|&&v| v).count()
    }

    pub fn select_hive_node(&mut self, id: &str, zoom: bool) {
        self.hive_selected_node_id = id.to_string();
        if let Some(n) = self.hive_nodes.iter().find(|n| n.id == id) {
            if zoom {
                self.hive_target_x = n.x;
                self.hive_target_y = n.y;
                self.hive_target_zoom = if id == "mind" { 1.0 } else { 1.48 };
            }
        }
    }
}
