use std::collections::BTreeMap;

use crate::wakan::wireless_nodes::distributed_dict_node::{
    Block, NodeAddress, Path, StartOfAddressRange, LEVELS, LEVEL_BUCKETS, ROOT_SIZE,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DistributedDict {
    // Dict stuff
    pub(super) root: [Block; ROOT_SIZE],
    pub(super) levels: [[(StartOfAddressRange, Block); LEVEL_BUCKETS]; LEVELS],
    pub(super) canopy: StartOfAddressRange,
    pub(super) tracked_nodes: BTreeMap<NodeAddress, Path>,
}
