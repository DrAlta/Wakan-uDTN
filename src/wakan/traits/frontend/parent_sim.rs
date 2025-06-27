use macroquad::prelude::*;

use crate::{
    graphic_frontend::{draw_graph_edges, draw_graph_nodes, draw_parent_arrow_heads},
    wakan::{Parent, Time, WakamSim, WirelessNode},
};

pub async fn tick_sim<P: std::fmt::Debug, N: WirelessNode<P> + Parent>(
    arrow_head_size: f32,
    node_size: f32,
    time_per_tick: f32,
    sim: &mut WakamSim<P, N>,
) {
    let mut time = 0.0;

    let mut sim_time: Time = 0;

    loop {
        clear_background(WHITE);

        draw_graph_edges(sim.get_graph());
        draw_graph_nodes(node_size, sim.get_graph());
        draw_parent_arrow_heads(arrow_head_size, node_size, sim.get_graph());

        time += get_frame_time();
        if time >= time_per_tick {
            time -= time_per_tick;
            sim.tick(sim_time);
            sim_time += 1;
        }
        //assert_ne!(sim_time, 10);
        next_frame().await
    }
}
