use macroquad::prelude::*;

use crate::{
    frontends::util::{draw_directed_edges, draw_graph_nodes},
    wakan::{Frontend, Time, VirtualVoteNode, VirtualVotePacket, WakamSim},
};

impl<
        MessageId: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug + std::hash::Hash + Clone,
        NodeAddress: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug + Clone,
    > Frontend
    for WakamSim<VirtualVotePacket<MessageId, NodeAddress>, VirtualVoteNode<MessageId, NodeAddress>>
{
    type Settings = (f32, f32, f32);
    const SETTINGS: Self::Settings = (7.0, 10.0, 0.0);
    async fn tick_sim(&mut self) {
        let (arrow_head_size, node_size, time_per_tick) = Self::SETTINGS;
        tick_sim(arrow_head_size, node_size, time_per_tick, self).await
    }
}

pub async fn tick_sim<MessageId: std::hash::Hash + Clone, NodeAddress: Clone>(
    arrow_head_size: f32,
    node_size: f32,
    time_per_tick: f32,
    sim: &mut WakamSim<
        VirtualVotePacket<MessageId, NodeAddress>,
        VirtualVoteNode<MessageId, NodeAddress>,
    >,
) where
    MessageId: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug,
    NodeAddress: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug,
{
    let mut time = 0.0;

    let mut sim_time: Time = 0;
    loop {
        clear_background(WHITE);

        draw_directed_edges(arrow_head_size, node_size, sim.get_graph());
        draw_graph_nodes(node_size, sim.get_graph());

        time += get_frame_time();
        if time >= time_per_tick {
            time -= time_per_tick;
            sim.tick(sim_time);
            sim_time += 1;
        }

        next_frame().await
    }
}
