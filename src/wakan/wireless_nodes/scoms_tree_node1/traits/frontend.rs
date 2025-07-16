use crate::{
    frontends::parent_sim,
    wakan::{Frontend, WakamSim},
};

use super::super::{ScomsTreeNode, ScomsTreePacket};

impl Frontend for WakamSim<ScomsTreePacket, ScomsTreeNode> {
    type Settings = (f32, f32, f32);
    const SETTINGS: Self::Settings = (7.0, 10.0, 0.0);
    async fn tick_sim(&mut self) {
        let (arrow_head_size, node_size, time_per_tick) = Self::SETTINGS;
        parent_sim::tick_sim::<ScomsTreePacket, ScomsTreeNode>(
            arrow_head_size,
            node_size,
            time_per_tick,
            self,
        )
        .await
    }
}
