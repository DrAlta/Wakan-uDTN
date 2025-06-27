use crate::wakan::{traits::parent_sim, Frontend, PlumTreeNode, PlumTreePacket, WakamSim};

impl Frontend for WakamSim<PlumTreePacket, PlumTreeNode> {
    type Settings = (f32, f32, f32);
    const SETTINGS: Self::Settings = (7.0, 10.0, 0.0);
    async fn tick_sim(&mut self) {
        let (arrow_head_size, node_size, time_per_tick) = Self::SETTINGS;
        parent_sim::tick_sim::<PlumTreePacket, PlumTreeNode>(
            arrow_head_size,
            node_size,
            time_per_tick,
            self,
        )
        .await
    }
}
