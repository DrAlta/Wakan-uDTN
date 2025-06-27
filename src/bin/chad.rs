use macroquad::prelude::*;
use std::env;

use wakan_sim::chad::{generate_random_graph, Graph, RawNode};
use wakan_sim::wakan::{
    /*FloodNode, FloodPacket,*/ Parent, PlumTreeNode, PlumTreePacket, Time, WakamSim,
    WirelessNode,
};

const TIME_PER_TICK: f32 = 0.5;
const NODE_SIZE: f32 = 10.0;

#[macroquad::main("Wakan Sim")]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    let graph = if let Some(filename) = args.get(1).cloned() {
        let json_str = std::fs::read_to_string(&filename)
            .unwrap_or_else(|_| panic!("Failed to read JSON file: {}", filename));
        let raw_nodes: Vec<RawNode> =
            serde_jsonrc::from_str(&json_str).expect("Invalid JSON format");

        Graph::<PlumTreePacket, PlumTreeNode>::from_raw_nodes(raw_nodes)
    } else {
        generate_random_graph(25, screen_width(), screen_height(), 15.0)
    };
    let mut time = 0.0;

    let mut sim_time: Time = 0;
    let mut sim = WakamSim::new(graph);

    loop {
        clear_background(WHITE);

        draw_graph_edges(sim.get_graph());
        draw_graph_nodes(sim.get_graph());
        draw_parent_arrow_heads(sim.get_graph());

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

fn draw_graph_nodes<P, N: WirelessNode<P>>(graph: &Graph<P, N>) {
    for node in graph.all_nodes() {
        draw_circle(node.x.into(), node.y.into(), NODE_SIZE, BLUE);
        draw_text(
            &format!("{}", node.id),
            Into::<f32>::into(node.x) + 12.0,
            node.y.into(),
            16.0,
            BLACK,
        );
    }
}
fn draw_graph_edges<P, N: WirelessNode<P>>(graph: &Graph<P, N>) {
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
}

fn draw_parent_arrow_heads<P, T: WirelessNode<P> + Parent>(graph: &Graph<P, T>) {
    let size = 7.0;
    for node in graph.all_nodes() {
        if let Some(parent_id) = node.outbound_links.get(0) {
            //if let Some(parent_id) = node.wireless_node.get_parent() {
            if let Some(target) = graph.get_node(&parent_id) {
                // Perpendicular vector (90° counter-clockwise)
                let head = vec2(node.x.into(), node.y.into());
                let tail = vec2(target.x.into(), target.y.into());
                let line = tail - head;
                let dir = line.normalize();
                let perp = Vec2::new(-dir.y, dir.x);

                let (arrow_tip, offset) = if false {
                    (tail - (dir * NODE_SIZE), tail - (dir * (size + NODE_SIZE)))
                } else {
                    (head + (dir * NODE_SIZE), head + (dir * (size + NODE_SIZE)))
                };
                let arrow_point = offset + (perp * size);
                draw_triangle(arrow_tip, arrow_point, offset, RED);
            }
        }
    }
}
