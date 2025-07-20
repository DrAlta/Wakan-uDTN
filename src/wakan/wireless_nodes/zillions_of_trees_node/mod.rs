//! if we want to merge ranges then hop count to the merged range should be the largest; as we want to make sure the true distance is at least that amount

mod structs;
pub use structs::{ZillionsOfTreesNode, ZillionsOfTreesPacket};

const MAX_AGE: u64 = 10;
