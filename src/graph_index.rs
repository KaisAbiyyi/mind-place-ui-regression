// src/graph_index.rs
// High-performance persistent indexed graph representation for Mind Place Hive Mind.

use crate::hive_data::HiveNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompactNode {
    pub id: String,
    pub title: String,
    pub layer: String,
    pub node_type: String,
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub is_root: bool,
    pub sources: usize,
    pub tag: String,
    pub branch_indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct IndexedEdge {
    pub source_idx: u32,
    pub target_idx: u32,
    /// Precomputed graph-space control point for the quadratic Bézier curve
    pub mx: f32,
    pub my: f32,
    /// Edge bounding box in graph-space for ultra-fast spatial culling
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

#[derive(Clone, Debug)]
pub struct SpatialGrid {
    pub cell_size: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Vec<u32>>,
}

impl SpatialGrid {
    pub fn build(nodes: &[CompactNode], cell_size: f32) -> Self {
        if nodes.is_empty() {
            return Self {
                cell_size,
                min_x: 0.0,
                min_y: 0.0,
                cols: 1,
                rows: 1,
                cells: vec![Vec::new()],
            };
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for n in nodes {
            min_x = min_x.min(n.x);
            min_y = min_y.min(n.y);
            max_x = max_x.max(n.x);
            max_y = max_y.max(n.y);
        }

        let width = (max_x - min_x).max(cell_size);
        let height = (max_y - min_y).max(cell_size);

        let cols = ((width / cell_size).ceil() as usize).max(1) + 1;
        let rows = ((height / cell_size).ceil() as usize).max(1) + 1;
        let mut cells = vec![Vec::new(); cols * rows];

        for (idx, n) in nodes.iter().enumerate() {
            let col = (((n.x - min_x) / cell_size).floor() as usize).min(cols - 1);
            let row = (((n.y - min_y) / cell_size).floor() as usize).min(rows - 1);
            cells[row * cols + col].push(idx as u32);
        }

        Self {
            cell_size,
            min_x,
            min_y,
            cols,
            rows,
            cells,
        }
    }

    pub fn query_range(&self, q_min_x: f32, q_min_y: f32, q_max_x: f32, q_max_y: f32) -> Vec<u32> {
        let mut result = Vec::new();
        if self.cells.is_empty() {
            return result;
        }

        let min_col = if q_min_x <= self.min_x {
            0
        } else {
            (((q_min_x - self.min_x) / self.cell_size).floor() as usize).min(self.cols - 1)
        };

        let max_col = if q_max_x <= self.min_x {
            0
        } else {
            (((q_max_x - self.min_x) / self.cell_size).floor() as usize).min(self.cols - 1)
        };

        let min_row = if q_min_y <= self.min_y {
            0
        } else {
            (((q_min_y - self.min_y) / self.cell_size).floor() as usize).min(self.rows - 1)
        };

        let max_row = if q_max_y <= self.min_y {
            0
        } else {
            (((q_max_y - self.min_y) / self.cell_size).floor() as usize).min(self.rows - 1)
        };

        for r in min_row..=max_row {
            for c in min_col..=max_col {
                let cell_idx = r * self.cols + c;
                result.extend_from_slice(&self.cells[cell_idx]);
            }
        }

        result
    }
}

#[derive(Clone, Debug)]
pub struct IndexedGraph {
    pub nodes: Vec<CompactNode>,
    pub id_to_index: HashMap<String, u32>,
    pub edges: Vec<IndexedEdge>,
    pub adjacency: Vec<Vec<u32>>,
    pub spatial_grid: SpatialGrid,
}

impl IndexedGraph {
    pub fn from_nodes_and_edges(raw_nodes: &[HiveNode], raw_edges: &[(String, String)]) -> Self {
        let mut id_to_index = HashMap::with_capacity(raw_nodes.len());
        for (i, n) in raw_nodes.iter().enumerate() {
            id_to_index.insert(n.id.clone(), i as u32);
        }

        let mut nodes: Vec<CompactNode> = Vec::with_capacity(raw_nodes.len());
        let mut adjacency: Vec<Vec<u32>> = vec![Vec::new(); raw_nodes.len()];

        for n in raw_nodes {
            let mut branch_indices = Vec::new();
            for b in &n.branches {
                if let Some(&b_idx) = id_to_index.get(b) {
                    branch_indices.push(b_idx);
                }
            }
            branch_indices.sort_unstable();
            branch_indices.dedup();

            nodes.push(CompactNode {
                id: n.id.clone(),
                title: n.title.clone(),
                layer: n.layer.clone(),
                node_type: n.node_type.clone(),
                x: n.x,
                y: n.y,
                r: n.r,
                is_root: n.id == "mind",
                sources: n.sources,
                tag: n.tag.clone(),
                branch_indices,
            });
        }

        let mut edges = Vec::with_capacity(raw_edges.len());
        for (src_id, tgt_id) in raw_edges {
            if let (Some(&u), Some(&v)) = (id_to_index.get(src_id), id_to_index.get(tgt_id)) {
                if u != v {
                    adjacency[u as usize].push(v);
                    adjacency[v as usize].push(u);

                    let pa_x = nodes[u as usize].x;
                    let pa_y = nodes[u as usize].y;
                    let pb_x = nodes[v as usize].x;
                    let pb_y = nodes[v as usize].y;

                    // Compute deterministic quadratic Bézier control point
                    let mx = (pa_x + pb_x) / 2.0 + (pb_y - pa_y) * 0.06;
                    let my = (pa_y + pb_y) / 2.0 - (pb_x - pa_x) * 0.06;

                    let min_x = pa_x.min(pb_x).min(mx);
                    let min_y = pa_y.min(pb_y).min(my);
                    let max_x = pa_x.max(pb_x).max(mx);
                    let max_y = pa_y.max(pb_y).max(my);

                    edges.push(IndexedEdge {
                        source_idx: u,
                        target_idx: v,
                        mx,
                        my,
                        min_x,
                        min_y,
                        max_x,
                        max_y,
                    });
                }
            }
        }

        for adj in &mut adjacency {
            adj.sort_unstable();
            adj.dedup();
        }

        let spatial_grid = SpatialGrid::build(&nodes, 250.0);

        Self {
            nodes,
            id_to_index,
            edges,
            adjacency,
            spatial_grid,
        }
    }

    /// Construct a fast boolean lookup mask of nodes belonging to the active selection set.
    /// activeSet = {selected} U branches(selected)
    pub fn compute_active_mask(&self, selected_idx: Option<u32>) -> Vec<bool> {
        let mut mask = vec![false; self.nodes.len()];
        if let Some(sel) = selected_idx {
            if let Some(node) = self.nodes.get(sel as usize) {
                mask[sel as usize] = true;
                for &b in &node.branch_indices {
                    if let Some(b_entry) = mask.get_mut(b as usize) {
                        *b_entry = true;
                    }
                }
            }
        }
        mask
    }
}
