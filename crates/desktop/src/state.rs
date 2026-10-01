use agent_core::agent::Agent;
use agent_core::note::Note;
use llm_ollama::client::OllamaClient;
use obsidian_store::vault::VaultStore;
use std::sync::Arc;

pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

pub struct GraphEdge {
    pub source: usize,
    pub target: usize,
}

#[derive(Default)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

pub struct ChatMessage {
    pub text: String,
    pub is_error: bool,
}

pub struct AppState {
    pub agent: Agent,
    pub models: Vec<String>,
    pub selected_model: String,
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub graph: GraphData,
    pub busy: bool,
}

impl AppState {
    pub fn new() -> Self {
        let llm = Arc::new(OllamaClient::new("http://localhost:11434"));
        let store = Arc::new(VaultStore::new("./vault"));
        let agent = Agent::new(llm, store, "llama3.1");

        Self {
            agent,
            models: vec![],
            selected_model: "llama3.1".into(),
            messages: vec![],
            input: String::new(),
            graph: GraphData::default(),
            busy: false,
        }
    }

    pub fn push_message(&mut self, text: impl Into<String>, is_error: bool) {
        self.messages.push(ChatMessage { text: text.into(), is_error });
    }

    /// Rebuilds the graph's node/edge lists from the vault, keeping existing
    /// node positions where possible so the layout doesn't jump on refresh.
    pub fn rebuild_graph(&mut self, titles: Vec<String>, links: Vec<(String, Vec<String>)>) {
        let mut old_positions: std::collections::HashMap<String, (f32, f32)> =
            self.graph.nodes.drain(..).map(|n| (n.id, (n.x, n.y))).collect();

        let nodes: Vec<GraphNode> = titles
            .into_iter()
            .map(|id| {
                let (x, y) = old_positions.remove(&id).unwrap_or_else(|| {
                    (rand_coord(800.0), rand_coord(600.0))
                });
                GraphNode { label: id.clone(), id, x, y, vx: 0.0, vy: 0.0 }
            })
            .collect();

        let index_of: std::collections::HashMap<&str, usize> =
            nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();

        let mut edges = vec![];
        for (from, link_list) in &links {
            let Some(&src) = index_of.get(from.as_str()) else { continue };
            for to in link_list {
                if let Some(&tgt) = index_of.get(to.as_str()) {
                    edges.push(GraphEdge { source: src, target: tgt });
                }
            }
        }

        self.graph = GraphData { nodes, edges };
    }

    pub fn step_layout(&mut self) {
        let n = self.graph.nodes.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (dx, dy) = {
                    let a = &self.graph.nodes[i];
                    let b = &self.graph.nodes[j];
                    (a.x - b.x, a.y - b.y)
                };
                let dist = dx.hypot(dy).max(1.0);
                let force = 6000.0 / (dist * dist);
                let (fx, fy) = (dx / dist * force, dy / dist * force);
                self.graph.nodes[i].vx += fx;
                self.graph.nodes[i].vy += fy;
                self.graph.nodes[j].vx -= fx;
                self.graph.nodes[j].vy -= fy;
            }
        }

        for e in &self.graph.edges {
            let (dx, dy) = {
                let a = &self.graph.nodes[e.source];
                let b = &self.graph.nodes[e.target];
                (b.x - a.x, b.y - a.y)
            };
            let dist = dx.hypot(dy).max(1.0);
            let force = (dist - 140.0) * 0.02;
            let (fx, fy) = (dx / dist * force, dy / dist * force);
            self.graph.nodes[e.source].vx += fx;
            self.graph.nodes[e.source].vy += fy;
            self.graph.nodes[e.target].vx -= fx;
            self.graph.nodes[e.target].vy -= fy;
        }

        for node in &mut self.graph.nodes {
            node.vx *= 0.85;
            node.vy *= 0.85;
            node.x += node.vx;
            node.y += node.vy;
        }
    }
}

fn rand_coord(max: f32) -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    (nanos as f32 / u32::MAX as f32) * max
}
