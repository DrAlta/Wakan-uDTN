//! Self-Stable Compact Minimum Stretch tree
//! pronounced like 'scones' but with a 'm' instead of 'n'
//!
//!
//!
//! todo:
//!
//! implment _A New Self-Stabilizing Minimum Spanning Tree Construction with Loop-free Property_
//!
//!
//! note we don't keep track of the lowest id known by a neighbor but the lowest id
//! accessing throu them we ignore ids that they can access thour us
//!
//! that means that if the low id they know of if throur us we won't find it
//!
//! that means is we are the parent of all are tree neighbors then are lowest known id
//! will be lower than any id we know are neighbors know about
mod find_lowest_id_lowest_accessable_thru_neighbor;
pub use find_lowest_id_lowest_accessable_thru_neighbor::find_lowest_id_lowest_accessable_thru_neighbor;
mod gen_next_heartbeat_time;
pub use gen_next_heartbeat_time::gen_next_heartbeat_time;
mod traits;

mod types;
pub use types::{ScomsTreeNode, ScomsTreePacket};

const MAX_AGE: u64 = 25;
