//! Self-Stable Compact Minimum Stretch tree
//! pronounced like 'scones' but with a 'm' instead of 'n'
//! 
//! todo:
//! we want to check is any of are neighbor now a node with a lower id than we do and if so make then are parent
mod find_lowest_id_lowest_accessable_thru_neighbor;
pub use find_lowest_id_lowest_accessable_thru_neighbor::find_lowest_id_lowest_accessable_thru_neighbor;
mod gen_next_heartbeat_time;
pub use gen_next_heartbeat_time::gen_next_heartbeat_time;
mod traits;

mod types;
use types::NeighborInfo;
pub use types::{ScomsTreeNode, ScomsTreePacket};

const MAX_AGE: u64 = 25;