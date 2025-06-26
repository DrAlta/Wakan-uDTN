use macroquad::prelude::*;
use std::env;

use wakan_sim::chad::{generate_random_graph, Graph, RawNode};
use wakan_sim::wakan::{FloodNode, FloodPacket, Time, WakamSim, WirelessNode};

const TIME_PER_TICK: f32 = 0.5;

#[macroquad::main("Wakan Sim")]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    let graph = if let Some(filename) = args.get(1).cloned() {
        let json_str = std::fs::read_to_string(&filename)
            .unwrap_or_else(|_| panic!("Failed to read JSON file: {}", filename));
        let raw_nodes: Vec<RawNode> =
            serde_jsonrc::from_str(&json_str).expect("Invalid JSON format");

        Graph::<FloodPacket, FloodNode>::from_raw_nodes(raw_nodes)
    } else {
        generate_random_graph(25, screen_width(), screen_height(), 15.0)
    };
    let mut time = 0.0;

    let mut sim_time: Time = 0;
    let mut sim = WakamSim::new(graph);

    loop {
        clear_background(WHITE);

        draw_graph(sim.get_graph());

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

fn draw_graph<P, N: WirelessNode<P>>(graph: &Graph<P, N>) {
    // Draw edges
    for node in graph.all_nodes() {
        for link_id in &node.outbound_links {
            if let Some(target) = graph.get_node(link_id) {
                draw_line(
                    node.x.into(),
                    node.y.into(),
                    target.x.into(),
                    target.y.into(),
                    2.0,
                    GRAY,
                );
            }
        }
    }

    // Draw nodes
    for node in graph.all_nodes() {
        draw_circle(node.x.into(), node.y.into(), 10.0, BLUE);
        draw_text(
            &format!("{}", node.id),
            Into::<f32>::into(node.x) + 12.0,
            node.y.into(),
            16.0,
            BLACK,
        );
    }
}
