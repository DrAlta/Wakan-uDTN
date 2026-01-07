use std::{
    array,
    collections::{BTreeMap, BTreeSet},
};

use crate::wakan::{
    wireless_nodes::distributed_dict_node::{
        distributed_dict_node::Block, NodeAddress, StartOfAddressRange, DEFAULT_HOP_LIMIT,
    },
    DistributedDictNode, NodeId,
};

impl DistributedDictNode {
    pub fn new(id: NodeId, _my_address: NodeAddress, canopy_start: StartOfAddressRange) -> Self {
        Self {
            id,
            _my_address,
            neighbors: BTreeSet::new(),
            root: array::from_fn(|_| Block::default()),
            levels: array::from_fn(|_| array::from_fn(|_| (0u64, Block::default()))),
            canopy: canopy_start,
            tracked_nodes: BTreeMap::new(),
            hop_limit: DEFAULT_HOP_LIMIT,
            last_hlu_received_time: 0,
            last_hlu_broadcast_time: 0,
            hlu_mean_interval: 0,
            hlu_variance: 0,
            time_of_next_hlu_broadcast: 0,
            need_to_broadcast_hlu: true,
        }
    }
}
