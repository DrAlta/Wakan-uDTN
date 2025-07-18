use crate::{
    frontends::cluster_sim,
    wakan::{Frontend, WakamSim},
};

use super::super::{LocalMinimumClusterNode, LocalMinimumClusterPacket};

impl Frontend for WakamSim<LocalMinimumClusterPacket, LocalMinimumClusterNode> {
    type Settings = (f32, f32, f32);
    const SETTINGS: Self::Settings = (7.0, 10.0, 0.0);
    async fn tick_sim(&mut self) {
        let (arrow_head_size, node_size, time_per_tick) = Self::SETTINGS;
        cluster_sim::tick_sim::<u64, LocalMinimumClusterPacket, LocalMinimumClusterNode>(
            arrow_head_size,
            node_size,
            time_per_tick,
            self,
        )
        .await
    }
}
