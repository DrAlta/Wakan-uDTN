use crate::wakan::wireless_nodes::distributed_dict_node::{HopLimitUpdate, DEFAULT_HOP_LIMIT};

impl HopLimitUpdate {
    pub fn new() -> Self {
        Self {
            hop_limit: DEFAULT_HOP_LIMIT,
            time_of_last_hlu_received: 0,
            time_of_last_hlu_broadcast: 0,
            hlu_mean_interval: 0,
            hlu_variance: 0,
        }
    }
}
