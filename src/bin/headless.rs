use std::env;

use wakan_sim::chad::{generate_random_graph, Graph, RawNode};
use wakan_sim::wakan::{FloodNode, FloodPacket, Time, WakamSim};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let graph = if let Some(filename) = args.get(1).cloned() {
        let json_str = std::fs::read_to_string(&filename)
            .unwrap_or_else(|_| panic!("Failed to read JSON file: {}", filename));
        let raw_nodes: Vec<RawNode> =
            serde_jsonrc::from_str(&json_str).expect("Invalid JSON format");

        Graph::<FloodPacket, FloodNode>::from_raw_nodes(raw_nodes)
    } else {
        generate_random_graph(25, 640.0, 480.0, 15.0)
    };

    let mut sim_time: Time = 0;
    let mut sim = WakamSim::new(graph);

    loop {
        sim.tick(sim_time);
        sim_time += 1;
        //assert_ne!( sim_time, 5)
        if sim.scheduled_receptions.is_empty() {
            break;
        }
    }
}
