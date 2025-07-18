use std::hash::{DefaultHasher, Hash, Hasher};

use crate::wakan::{
    scoms_tree_node2::{ScomsTreeNode, MAX_AGE},
    Time, SIM_SIZE,
};
impl ScomsTreeNode {
    pub fn gen_next_heartbeat_time(&self, now: Time) -> Time {
        if self.id.0 as u64 == now % SIM_SIZE as u64 {
            return MAX_AGE + 5;
        };
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        self.id.0.hash(&mut hasher);
        let hash = hasher.finish();
        (MAX_AGE.saturating_sub((hash % 10) as Time)) + now
    }
}
