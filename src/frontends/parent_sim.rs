use std::collections::BTreeSet;

use macroquad::prelude::*;
use qol::logy;

use crate::{
    detect_cycles::detect_cycles_with_roots,
    graphic_frontend::{draw_graph_edges, draw_graph_nodes, draw_parent_visuals},
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
    let mut last_roots = BTreeSet::new();
    loop {
        clear_background(WHITE);

        draw_graph_edges(sim.get_graph());
        draw_graph_nodes(node_size, sim.get_graph());
        draw_parent_visuals(arrow_head_size, node_size, sim.get_graph());

        time += get_frame_time();
        if time >= time_per_tick {
            time -= time_per_tick;
            sim.tick(sim_time);
            sim_time += 1;
        }
        //assert_ne!(sim_time, 10);

        let x = sim
            .get_graph()
            .all_nodes()
            .filter_map(|node| Some((node.id.clone(), node.wireless_node.get_parent()?)))
            .collect();
        let (cycles_ka, new_roots) = detect_cycles_with_roots(&x);
        if new_roots != last_roots {
            logy!(
                "info",
                "\nnew roots:{new_roots:?}(note:doesn't include lone nodes)"
            );
            last_roots = new_roots;
        }
        if cycles_ka {
            logy!("info", "\nCycles found!");
        }
        next_frame().await
    }
}
