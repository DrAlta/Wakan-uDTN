use macroquad::prelude::*;
use std::env;

use wakan_sim::wakan::{Frontend, Graph, ScomsTreeNode, ScomsTreePacket, RawNode, WakamSim};

#[macroquad::main("Wakan Sim")]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    let graph: Graph<ScomsTreePacket, ScomsTreeNode> = if let Some(filename) = args.get(1).cloned() {
        let json_str = std::fs::read_to_string(&filename)
            .unwrap_or_else(|_| panic!("Failed to read JSON file: {}", filename));
        let raw_nodes: Vec<RawNode> =
            serde_jsonrc::from_str(&json_str).expect("Invalid JSON format");

        Graph::from_raw_nodes(raw_nodes)
    } else {
        Graph::generate_random_graph(25, screen_width(), screen_height(), 15.0)
    };
    let mut sim = WakamSim::new(graph);

    loop {
        sim.tick_sim().await
    }
}
