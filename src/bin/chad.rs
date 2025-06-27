use macroquad::prelude::*;
use std::env;

use wakan_sim::{
    graphic_frontend::{draw_graph_edges, draw_graph_nodes, draw_parent_arrow_heads},
    wakan::{
        /*FloodNode, FloodPacket,*/ Graph, Parent, PlumTreeNode, PlumTreePacket, RawNode,
        Time, WakamSim, WirelessNode,
    },
};

const TIME_PER_TICK: f32 = 0.5;
const NODE_SIZE: f32 = 10.0;

#[macroquad::main("Wakan Sim")]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    let graph: Graph<PlumTreePacket, PlumTreeNode> = if let Some(filename) = args.get(1).cloned() {
        let json_str = std::fs::read_to_string(&filename)
            .unwrap_or_else(|_| panic!("Failed to read JSON file: {}", filename));
        let raw_nodes: Vec<RawNode> =
            serde_jsonrc::from_str(&json_str).expect("Invalid JSON format");

        Graph::from_raw_nodes(raw_nodes)
    } else {
        Graph::generate_random_graph(25, screen_width(), screen_height(), 15.0)
    };
    let mut time = 0.0;

    let mut sim_time: Time = 0;
    let mut sim = WakamSim::new(graph);

    loop {
        clear_background(WHITE);

        draw_graph_edges(sim.get_graph());
        draw_graph_nodes(NODE_SIZE, sim.get_graph());
        draw_parent_arrow_heads(7.0, NODE_SIZE, sim.get_graph());

        time += get_frame_time();
        if time >= TIME_PER_TICK {
            time -= TIME_PER_TICK;
            sim.tick(sim_time);
            sim_time += 1;
        }
        //assert_ne!(sim_time, 10);
        next_frame().await
    }
}
