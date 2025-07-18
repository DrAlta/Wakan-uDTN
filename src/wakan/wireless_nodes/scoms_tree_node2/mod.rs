//! Self-Stable Compact Minimum Stretch tree
//! pronounced like 'scones' but with a 'm' instead of 'n'
//!
//!
//! * maybe use the neighbor's neihbor's lowest neighbor to make clusters?
//! ** you can find the lowest nodeId that is 3 links away
//! *** find the ones that share it
//! **** The 'prince's likewize the highest shared neighbor's neihbor's neihbor is their 'princess'
//! *** senda message to the prince(ss) and they can bounce it out in {prince: NodeId, subject: NodeId}
//! **** needs to travel prince -> a -> b -> subject
//! ***** retransmit if prince is my neighbor or a neighbot of my neighbor
//!
//!
//! * In particular, Maccari and Cigno [ Pop-routing: Centrality-based tuning of control messages for faster route convergence.] focused on optimizing the
//! *: frequency of hello messages for link-sensing in wireless networks, and showed that the optimal
//! *: frequency f(v) at which every node v must sense its neighbors is
//! ** use that for heatbeat?
//!
//!  todo:
//!   b
//!  / \
//! a---c---d---e
//!  c is shortest path from a to e
//!  b isn't on shortst path from a to e
//! b on 0 shorst paths to c
//! b on 0 shorthpaths throu a to c
//!
//! =============
//!
//!   b   d
//!  / \ / \
//! a---c---e  
//!  c is shortest path from a to e
//!  b isn't on shortst path from a to e
//!
//! ================
//!
//!           d
//!          / \
//! a---b---c---e
//!  c is shortest path from a to e
//!  b is on shortst path from a to e
//!
//! ==================
//!
//!  
//!
//! a---b---c---d---e
//!  c is shortest path from a to e
//!  b is on shortst path from a to e
//!
//! ==================
//! a-b-c-d-e
//! the number of shorst paths(NofSP) from a to e is `NofSP(a,e) = NofSP(a,c) * NofSP(c, e)`
//! what is b's contribution to `NofSP(a,e)`?
//! b's contribiton to  NofSP(a,c) <something something> c's contribution to  NofSP(b,d)
//!
//! * b needs to know the number of shorest paths thru c that don't go thru b
//! ** `number_of_shorst_paths from a to e that run thru b should be  NofSP(a,e) / NofSP(c,e)
//! ** @ if b a short from a to c then b contribies a's contribution of shortest pathsto b + 1 to c @
//! *** `b_contribution_to_c = if b_shorts_a_and_c {a_contribution_to_b + 1} else {0}`
mod find_lowest_id_lowest_accessible_thru_neighbor;
pub use find_lowest_id_lowest_accessible_thru_neighbor::find_lowest_id_lowest_accessible_thru_neighbor;
mod gen_next_heartbeat_time;
mod traits;

mod types;
pub use types::{ScomsTreeNode, ScomsTreePacket};

const MAX_AGE: u64 = 25;
