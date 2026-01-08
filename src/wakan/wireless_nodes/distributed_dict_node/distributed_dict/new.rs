use std::{array, collections::BTreeMap};

use crate::wakan::wireless_nodes::distributed_dict_node::{
    Block, DistributedDict, StartOfAddressRange,
};

impl DistributedDict {
    pub fn new(canopy_start: StartOfAddressRange) -> Self {
        Self {
            root: array::from_fn(|_| Block::default()),
            levels: array::from_fn(|_| array::from_fn(|_| (0u64, Block::default()))),
            canopy: canopy_start,
            tracked_nodes: BTreeMap::new(),
        }
    }
}
