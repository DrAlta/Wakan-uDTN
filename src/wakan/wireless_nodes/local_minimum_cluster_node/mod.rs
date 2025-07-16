mod find_lowest_id_lowest_accessable_thru_neighbor;
pub use find_lowest_id_lowest_accessable_thru_neighbor::find_lowest_id_lowest_accessable_thru_neighbor;
mod gen_next_heartbeat_time;
pub use gen_next_heartbeat_time::gen_next_heartbeat_time;
mod traits;

mod types;
pub use types::{LocalMinimumClusterNode, LocalMinimumClusterPacket};

const MAX_AGE: u64 = 25;
