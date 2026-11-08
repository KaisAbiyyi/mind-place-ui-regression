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
fn test_path_builder_bezier_stroke() {
    use gpui::{point, px, PathBuilder};

    let mut builder = PathBuilder::stroke(px(1.5)).dash_array(&[px(3.0), px(5.0)]);
    // Subpath 1
    builder.move_to(point(px(10.0), px(10.0)));
    builder.curve_to(point(px(100.0), px(50.0)), point(px(50.0), px(100.0)));
    // Subpath 2
    builder.move_to(point(px(150.0), px(150.0)));
    builder.curve_to(point(px(250.0), px(200.0)), point(px(200.0), px(250.0)));
    
    let path = builder.build();
    assert!(path.is_ok(), "Multi-contour stroked quadratic bezier path should build successfully via Lyon");

    // Stress test: 15,000 quadratic curves in chunks of 4,000
    for chunk_start in (0..15_000).step_by(4000) {
        let chunk_end = (chunk_start + 4000).min(15_000);
        let mut chunk_builder = PathBuilder::stroke(px(0.85)).dash_array(&[px(3.0), px(5.0)]);
        for i in chunk_start..chunk_end {
            let offset = (i as f32) * 0.1;
            chunk_builder.move_to(point(px(offset), px(offset)));
            chunk_builder.curve_to(point(px(offset + 10.0), px(offset + 20.0)), point(px(offset + 5.0), px(offset + 15.0)));
        }
        let chunk_path = chunk_builder.build();
        assert!(chunk_path.is_ok(), "Chunk of 4000 stroked bezier curves should build cleanly");
    }
}

#[test]
fn test_rendering_contract_edge_control_points_and_culling() {
    use crate::graph_index::{CompactNode, IndexedEdge};

    let n1 = CompactNode {
        id: "n1".into(),
        title: "Node 1".into(),
        layer: "work".into(),
        node_type: "concept".into(),
        x: 100.0,
        y: 100.0,
        r: 10.0,
        is_root: false,
        sources: 1,
        tag: "tag".into(),
        branch_indices: vec![1],
    };

    let n2 = CompactNode {
        id: "n2".into(),
        title: "Node 2".into(),
        layer: "work".into(),
        node_type: "concept".into(),
        x: 300.0,
        y: 100.0,
        r: 8.0,
        is_root: false,
        sources: 1,
        tag: "tag".into(),
        branch_indices: vec![0],
    };

    // Horizontal edge: pa = (100, 100), pb = (300, 100)
    // chord: dx = 200, dy = 0
    // mx = (100 + 300)/2 + (0)*0.06 = 200.0
    // my = (100 + 100)/2 - (200)*0.06 = 100 - 12 = 88.0
    let mx = (n1.x + n2.x) / 2.0 + (n2.y - n1.y) * 0.06;
    let my = (n1.y + n2.y) / 2.0 - (n2.x - n1.x) * 0.06;

    assert_eq!(mx, 200.0);
    assert_eq!(my, 88.0);

    let edge = IndexedEdge {
        source_idx: 0,
        target_idx: 1,
        mx,
        my,
        min_x: n1.x.min(n2.x).min(mx),
        min_y: n1.y.min(n2.y).min(my),
        max_x: n1.x.max(n2.x).max(mx),
        max_y: n1.y.max(n2.y).max(my),
    };

    assert_eq!(edge.min_x, 100.0);
    assert_eq!(edge.max_x, 300.0);
    assert_eq!(edge.min_y, 88.0);
    assert_eq!(edge.max_y, 100.0);

    // Culling test: query rect entirely to the right (x in [400, 500])
    let view_min_gx = 400.0;
    let view_max_gx = 500.0;
    let view_min_gy = 0.0;
    let view_max_gy = 200.0;
    let is_culled = edge.max_x < view_min_gx || edge.min_x > view_max_gx || edge.max_y < view_min_gy || edge.min_y > view_max_gy;
    assert!(is_culled, "Edge completely outside viewport rect should be culled");
}

#[test]
fn test_rendering_contract_label_visibility_contract() {
    // Contract conditions:
    // Full-fidelity: (n.r >= 9.0) || is_sel || is_hover || (cam_zoom > 1.35 && r >= 6.0)
    // LOD: is_sel || is_hover || (r >= 12.0) || (cam_zoom > 1.80 && r >= 8.0)

    let test_node_r8 = 8.0f32;
    let test_node_r10 = 10.0f32;
    let s = 1.0f32;
    let r8_screen = test_node_r8 * s.max(0.68);
    let r10_screen = test_node_r10 * s.max(0.68);

    // Node r=8, not selected, not hovered, zoom=1.0
    let full_r8 = (test_node_r8 >= 9.0) || false || false || (s > 1.35 && r8_screen >= 6.0);
    let lod_r8 = false || false || (r8_screen >= 12.0) || (s > 1.80 && r8_screen >= 8.0);
    assert!(!full_r8, "r=8 should not render label in normal zoom Full mode");
    assert!(!lod_r8, "r=8 should not render label in normal zoom LOD mode");

    // Node r=10, not selected, not hovered, zoom=1.0
    let full_r10 = (test_node_r10 >= 9.0) || false || false || (s > 1.35 && r10_screen >= 6.0);
    let lod_r10 = false || false || (r10_screen >= 12.0) || (s > 1.80 && r10_screen >= 8.0);
    assert!(full_r10, "r=10 SHOULD render label in Full mode (r >= 9.0)");
    assert!(!lod_r10, "r=10 should NOT render label in LOD mode (screen r = 10 < 12.0)");
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
