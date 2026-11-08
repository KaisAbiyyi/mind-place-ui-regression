use crate::app_state::AppState;
use crate::hive_data;
use crate::models::Page;
use crate::theme::ThemeMode;

#[test]
fn test_all_11_pages_navigation() {
    let mut state = AppState::new();
    let pages = [
        Page::Home,
        Page::Conversations,
        Page::Memory,
        Page::Hive,
        Page::Files,
        Page::Providers,
        Page::Setup,
        Page::Settings,
        Page::ConversationDetail,
        Page::CollectionDetail,
        Page::MemoryDetail,
    ];

    assert_eq!(pages.len(), 11);
    for page in pages {
        state.navigate(page);
        assert_eq!(state.active_page, page);
        assert!(!page.name().is_empty());
    }
}

#[test]
fn test_theme_and_modes() {
    let mut state = AppState::new();
    assert_eq!(state.theme.mode, ThemeMode::Light);

    state.toggle_theme();
    assert_eq!(state.theme.mode, ThemeMode::Dark);

    state.toggle_theme();
    assert_eq!(state.theme.mode, ThemeMode::Light);

    assert!(!state.tech_mode);
    state.toggle_tech_mode();
    assert!(state.tech_mode);

    assert!(!state.sidebar_collapsed);
    state.toggle_sidebar();
    assert!(state.sidebar_collapsed);
}

#[test]
fn test_hive_graph_dataset() {
    let nodes = hive_data::get_hive_nodes();
    assert_eq!(nodes.len(), 33, "Hive graph must have 33 nodes");
    assert_eq!(
        hive_data::HIVE_EDGES.len(),
        69,
        "Hive graph must have 69 edges"
    );

    // Center node
    let center = nodes
        .iter()
        .find(|n| n.id == "mind")
        .expect("Center node 'mind' must exist");
    assert_eq!(center.x, 600.0);
    assert_eq!(center.y, 400.0);

    // Verify all edge endpoints exist in nodes
    for (source, target) in hive_data::HIVE_EDGES {
        assert!(
            nodes.iter().any(|n| n.id == *source),
            "Source node {} missing",
            source
        );
        assert!(
            nodes.iter().any(|n| n.id == *target),
            "Target node {} missing",
            target
        );
    }
}

#[test]
fn test_curated_memories_and_collections() {
    let state = AppState::new();
    assert_eq!(state.collections.len(), 6);
    assert_eq!(state.memories.len(), 18);

    for mem in &state.memories {
        assert!(!mem.title.is_empty());
        assert!(!mem.content.is_empty());
        assert!(!mem.source.is_empty());
        assert!(!mem.why.is_empty());
        assert!(!mem.score.is_empty());
    }
}

#[test]
fn test_vault_and_folders() {
    let mut state = AppState::new();
    assert!(!state.vault_locked);
    state.lock_vault();
    assert!(state.vault_locked);
    state.unlock_vault();
    assert!(!state.vault_locked);

    assert_eq!(state.approved_folders.len(), 3);
    assert_eq!(state.total_files(), 56);
    assert_eq!(state.watching_folder_count(), 2);
    assert_eq!(state.paused_folder_count(), 1);
}

#[test]
fn test_providers_and_boundaries() {
    let state = AppState::new();
    assert_eq!(state.connected_providers.len(), 3);
    assert_eq!(state.connected_provider_count(), 1);
    assert_eq!(state.vault_boundaries.len(), 4);
}

#[test]
fn test_conversations_archive() {
    let state = AppState::new();
    assert_eq!(state.conversations.len(), 8);
    for conv in &state.conversations {
        assert!(!conv.title.is_empty());
        assert!(!conv.provider.is_empty());
        assert!(!conv.collection.is_empty());
    }
}
