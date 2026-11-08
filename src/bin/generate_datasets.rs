// src/bin/generate_datasets.rs
// Deterministic graph dataset generator for Mind Place GPUI vs WebView2 benchmark.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }

    fn range_usize(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        min + (self.next_u64() as usize % (max - min))
    }

    // Box-Muller transform for gaussian noise
    fn next_gaussian(&mut self, mean: f32, std_dev: f32) -> f32 {
        let u1 = self.next_f32().max(1e-7);
        let u2 = self.next_f32();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
        mean + z0 * std_dev
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub layer: String,
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub sub: String,
    pub summary: String,
    pub sources: usize,
    pub branches: Vec<String>,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphDataset {
    pub name: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub density: f64,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<[String; 2]>,
}

struct ClusterDef {
    name: &'static str,
    layer: &'static str,
    node_type: &'static str,
    tag_prefix: &'static str,
    base_dist: f32,
    angle: f32,
}

const CLUSTERS: &[ClusterDef] = &[
    ClusterDef {
        name: "Central Intelligence",
        layer: "all",
        node_type: "Core intelligence",
        tag_prefix: "ROOT",
        base_dist: 0.0,
        angle: 0.0,
    },
    ClusterDef {
        name: "Agent Memory & Context",
        layer: "research",
        node_type: "System",
        tag_prefix: "SYSTEM / MEMORY",
        base_dist: 280.0,
        angle: -2.2,
    },
    ClusterDef {
        name: "Connectivity & MCP",
        layer: "research",
        node_type: "System",
        tag_prefix: "SYSTEM / CONNECTIVITY",
        base_dist: 310.0,
        angle: -0.7,
    },
    ClusterDef {
        name: "Secure Vault & Keys",
        layer: "research",
        node_type: "System",
        tag_prefix: "SYSTEM / TRUST",
        base_dist: 320.0,
        angle: 0.1,
    },
    ClusterDef {
        name: "Conversation Archive",
        layer: "research",
        node_type: "Knowledge",
        tag_prefix: "KNOWLEDGE / ARCHIVE",
        base_dist: 290.0,
        angle: 2.9,
    },
    ClusterDef {
        name: "Engineering Workspace",
        layer: "work",
        node_type: "Project",
        tag_prefix: "PROJECT / WORK",
        base_dist: 360.0,
        angle: 0.9,
    },
    ClusterDef {
        name: "Cross-Project Insights",
        layer: "research",
        node_type: "Idea",
        tag_prefix: "INSIGHT / CROSS",
        base_dist: 300.0,
        angle: 1.8,
    },
    ClusterDef {
        name: "Learning & Practice",
        layer: "learning",
        node_type: "Idea",
        tag_prefix: "LEARNING",
        base_dist: 450.0,
        angle: -0.4,
    },
    ClusterDef {
        name: "Personal Context & Life",
        layer: "life",
        node_type: "Knowledge",
        tag_prefix: "LIFE / PERSONAL",
        base_dist: 460.0,
        angle: 3.8,
    },
];

const PREFIXES: &[&str] = &[
    "Context", "Vector", "Retrieval", "Provenance", "Cache", "Neural", "Memory", "Vault", "Index",
    "Session", "Topology", "Protocol", "Agent", "Cluster", "Kernel", "Trace", "Signal", "Schema",
    "Sync", "Pipeline", "Ledger", "Query", "Semantic", "Spatial", "Token", "Graph", "Embedding",
    "Runtime", "Broker", "Worker", "Shard", "Anchor", "Branch", "Relay", "Beacon", "Mirror",
];

const SUFFIXES: &[&str] = &[
    "Engine", "Registry", "Storage", "Adapter", "Graph", "Layer", "Router", "Buffer", "Ledger",
    "Gateway", "Matrix", "Runtime", "Observer", "Parser", "Dispatcher", "Bridge", "Broker",
    "Collector", "Model", "Resolver", "Controller", "Service", "Fabric", "Filter", "Nexus",
    "Stream", "Hub", "Node", "Link", "Vault", "Index", "Core", "View", "Tree",
];

fn generate_dataset(total_nodes: usize, seed: u64) -> GraphDataset {
    let mut rng = Rng::new(seed);
    let scale = (total_nodes as f32 / 33.0).sqrt().max(1.0);

    // Calculate cluster centers
    let cluster_centers: Vec<(f32, f32)> = CLUSTERS
        .iter()
        .map(|c| {
            if c.base_dist == 0.0 {
                (600.0, 400.0)
            } else {
                let dist = c.base_dist * scale;
                let x = 600.0 + c.angle.cos() * dist;
                let y = 400.0 + c.angle.sin() * dist;
                (x, y)
            }
        })
        .collect();

    let mut nodes: Vec<GraphNode> = Vec::with_capacity(total_nodes);

    // 1. Generate Nodes
    for i in 0..total_nodes {
        let cluster_idx = if i == 0 {
            0 // Node 0 is always root Mind Place
        } else {
            rng.range_usize(0, CLUSTERS.len())
        };

        let cluster = &CLUSTERS[cluster_idx];
        let (cx, cy) = cluster_centers[cluster_idx];

        // Spatial spread within cluster
        let spread_std = 70.0 * scale.powf(0.8);
        let x = rng.next_gaussian(cx, spread_std);
        let y = rng.next_gaussian(cy, spread_std);

        let is_root = i == 0;
        let is_hub = !is_root && (i % 25 == 0 || (i < 20 && i % 4 == 0));

        let r = if is_root {
            18.0
        } else if is_hub {
            rng.range_f32(13.0, 16.0)
        } else if i % 7 == 0 {
            rng.range_f32(10.0, 12.0)
        } else {
            rng.range_f32(7.5, 9.5)
        };

        let pref = PREFIXES[rng.range_usize(0, PREFIXES.len())];
        let suff = SUFFIXES[rng.range_usize(0, SUFFIXES.len())];
        let title = if is_root {
            "Mind Place".to_string()
        } else {
            format!("{} {}", pref, suff)
        };

        let id = if is_root {
            "mind".to_string()
        } else {
            format!("node_{}", i)
        };

        let node_type = if is_root {
            "Core intelligence".to_string()
        } else {
            cluster.node_type.to_string()
        };

        let layer = if is_root {
            "all".to_string()
        } else {
            cluster.layer.to_string()
        };

        let sources = if is_root {
            42
        } else {
            rng.range_usize(1, 35)
        };

        let sub = format!("{} component tracking {} sources.", cluster.name, sources);
        let summary = format!(
            "Node {} connects {} intelligence into the active Mind Place spatial index.",
            id, cluster.name
        );
        let tag = format!("{} / {}", cluster.tag_prefix, id.to_uppercase());

        nodes.push(GraphNode {
            id,
            title,
            node_type,
            layer,
            x,
            y,
            r,
            sub,
            summary,
            sources,
            branches: Vec::new(),
            tag,
        });
    }

    // 2. Generate Edges
    // Target average degree: ~2.5 to 3.5 edges per node
    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();
    let isolated_ratio = 0.05; // 5% isolated nodes
    let isolated_cutoff = (total_nodes as f32 * (1.0 - isolated_ratio)) as usize;

    // A. Connect Root (node 0) to primary hubs
    for i in 1..total_nodes.min(12) {
        let (u, v) = (0, i);
        edge_set.insert((u.min(v), u.max(v)));
    }

    // B. Intra-cluster spatial proximity edges
    // For each node up to isolated_cutoff, find closest 2-4 candidates in same or neighboring spatial grid
    let step_stride = (total_nodes / 200).max(1);
    for i in 1..isolated_cutoff {
        let target_edges = rng.range_usize(2, 5);
        let mut connected = 0;

        // Search a local window around i
        let min_j = if i > 40 { i - 40 } else { 0 };
        let max_j = (i + 40).min(isolated_cutoff);

        for j in (min_j..max_j).step_by(step_stride) {
            if i == j {
                continue;
            }
            let dx = nodes[i].x - nodes[j].x;
            let dy = nodes[i].y - nodes[j].y;
            let dist_sq = dx * dx + dy * dy;

            // Distance threshold depends on scale
            let threshold = (180.0 * scale).powi(2);
            if dist_sq < threshold {
                let pair = (i.min(j), i.max(j));
                if edge_set.insert(pair) {
                    connected += 1;
                    if connected >= target_edges {
                        break;
                    }
                }
            }
        }

        // Fallback: connect to at least one nearby node if still 0
        if connected == 0 && i > 1 {
            let j = rng.range_usize(1, i);
            edge_set.insert((j.min(i), j.max(i)));
        }
    }

    // C. Cross-cluster bridge edges (~15% of edges)
    let bridge_count = (total_nodes as f32 * 0.45) as usize;
    for _ in 0..bridge_count {
        let u = rng.range_usize(1, isolated_cutoff);
        let v = rng.range_usize(1, isolated_cutoff);
        if u != v {
            edge_set.insert((u.min(v), u.max(v)));
        }
    }

    // Convert to string edge pairs and populate node branches deterministically
    let mut sorted_edge_pairs: Vec<(usize, usize)> = edge_set.into_iter().collect();
    sorted_edge_pairs.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut edges: Vec<[String; 2]> = Vec::with_capacity(sorted_edge_pairs.len());
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();

    for (u, v) in sorted_edge_pairs {
        let id_u = nodes[u].id.clone();
        let id_v = nodes[v].id.clone();
        adj.entry(id_u.clone()).or_default().push(id_v.clone());
        adj.entry(id_v.clone()).or_default().push(id_u.clone());
        edges.push([id_u, id_v]);
    }

    // Set branches for each node (sorted deterministically, top 8)
    for node in &mut nodes {
        if let Some(neighbors) = adj.get(&node.id) {
            let mut nb = neighbors.clone();
            nb.sort();
            nb.truncate(8);
            node.branches = nb;
        }
    }

    let edge_count = edges.len();
    let density = if total_nodes > 1 {
        (2.0 * edge_count as f64) / (total_nodes as f64 * (total_nodes as f64 - 1.0))
    } else {
        0.0
    };

    GraphDataset {
        name: format!("graph_{}", total_nodes),
        node_count: total_nodes,
        edge_count,
        density,
        nodes,
        edges,
    }
}

fn compute_sha256(bytes: &[u8]) -> String {
    // FNV-1a + robust 64-bit multi-round digest for manifest validation without external crypto crate
    let mut h1: u64 = 0xcbf29ce484222325;
    let mut h2: u64 = 0x100000001b3;
    for &b in bytes {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x100000001b3);
        h2 = (h2 ^ (b as u64)).rotate_left(5).wrapping_add(h1);
    }
    format!("{:016x}{:016x}", h1, h2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_dir = if args.len() > 1 {
        std::path::PathBuf::from(&args[1])
    } else {
        std::path::PathBuf::from("comparison/benchmarks/phase3/datasets")
    };

    create_dir_all(&target_dir).expect("failed to create datasets dir");

    let sizes = [100, 1000, 10000, 25000];
    let mut manifest_entries = Vec::new();

    for &size in &sizes {
        println!("Generating Phase 3 dataset: {} nodes...", size);
        let ds = generate_dataset(size, 0x4D494E44_504C4143 ^ (size as u64));
        let filename = format!("graph_{}.json", size);
        let path = target_dir.join(&filename);

        let json_str = serde_json::to_string_pretty(&ds).expect("failed to serialize");
        let hash = compute_sha256(json_str.as_bytes());
        let mut file = File::create(&path).expect("failed to create file");
        file.write_all(json_str.as_bytes())
            .expect("failed to write");

        let avg_degree = (2.0 * ds.edge_count as f64) / (ds.node_count as f64);
        println!(
            "  -> Wrote {} ({} nodes, {} edges, avg degree: {:.3}, hash: {}, {:.2} MB)",
            filename,
            ds.node_count,
            ds.edge_count,
            avg_degree,
            hash,
            json_str.len() as f64 / 1_048_576.0
        );

        manifest_entries.push(serde_json::json!({
            "filename": filename,
            "node_count": ds.node_count,
            "edge_count": ds.edge_count,
            "avg_degree_2E_per_N": avg_degree,
            "density": ds.density,
            "hash": hash,
            "size_bytes": json_str.len()
        }));
    }

    let manifest_path = target_dir.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest_entries).unwrap();
    File::create(manifest_path).unwrap().write_all(manifest_json.as_bytes()).unwrap();

    println!("All Phase 3 deterministic datasets & manifest successfully generated in {:?}!", target_dir);
}
