#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Home,
    Conversations,
    Memory,
    Hive,
    Files,
    Providers,
    Setup,
    Settings,
    ConversationDetail,
    CollectionDetail,
    MemoryDetail,
}

impl Page {
    pub fn name(&self) -> &'static str {
        match self {
            Page::Home => "Overview",
            Page::Conversations => "Conversations Archive",
            Page::Memory => "Curated Memory",
            Page::Hive => "Hive Mind Graph",
            Page::Files => "Files & Secure Vault",
            Page::Providers => "AI Providers & MCP",
            Page::Setup => "Setup & Onboarding",
            Page::Settings => "Settings",
            Page::ConversationDetail => "Conversation Reader",
            Page::CollectionDetail => "Collection Inspector",
            Page::MemoryDetail => "Memory Provenance",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Conversation {
    pub title: &'static str,
    pub provider: &'static str,
    pub collection: &'static str,
    pub timestamp: &'static str,
    pub sha: &'static str,
    pub msgs: usize,
}

#[derive(Clone, Debug)]
pub struct MemoryCollection {
    pub name: &'static str,
    pub count: usize,
    pub desc: &'static str,
    pub pct: usize,
}

#[derive(Clone, Debug)]
pub struct CuratedMemory {
    pub id: String,
    pub collection: &'static str,
    pub title: &'static str,
    pub content: &'static str,
    pub source: &'static str,
    pub mem_type: &'static str,
    pub updated: &'static str,
    pub why: &'static str,
    pub score: &'static str,
}

#[derive(Clone, Debug)]
pub struct SampleFile {
    pub name: String,
    pub size: String,
    pub extractor: String,
    pub status: String,
    pub hash: String,
}

#[derive(Clone, Debug)]
pub struct ApprovedFolder {
    pub id: String,
    pub path: String,
    pub collection: String,
    pub files_count: usize,
    pub size_mb: f64,
    pub formats: Vec<String>,
    pub status: String, // "Watching" or "Paused"
    pub recursive: String,
    pub last_synced: String,
    pub dev_provenance: String,
    pub sample_files: Vec<SampleFile>,
    pub scanning: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConvTabFilter {
    All,
    Starred,
    Archived,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FolderStatusFilter {
    All,
    Watching,
    Paused,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemoryViewTab {
    Collections,
    All,
}

#[derive(Clone, Debug)]
pub struct DialogModal {
    pub kicker: String,
    pub title: String,
    pub body_html: String,
}
