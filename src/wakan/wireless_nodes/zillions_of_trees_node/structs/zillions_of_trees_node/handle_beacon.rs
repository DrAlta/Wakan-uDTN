use std::collections::{BTreeMap, BTreeSet};

use crate::wakan::{wireless_nodes::zillions_of_trees_node::structs::NeighborInfo, NodeId, Radio, Time};

use super::super::super::ZillionsOfTreesNode;

impl ZillionsOfTreesNode{
    pub fn handle_beacon(
        &mut self,
        neighbor_id: &NodeId,
        neighbors_neighbors: &BTreeMap<NodeId, BTreeSet<NodeId>>,
        princess: &NodeId,
        recieved_time: Time,
        radio: &Radio,
    ){
        let mut queen_inner = neighbor_id.0;

        let mut tree = BTreeSet::new();
        let mut flow = BTreeSet::new();

        for (neighbors_neighbor_id, accessible_thru_neighbors_neighbor) in neighbors_neighbors {
            if neighbors_neighbor_id.0 != self.id.0 {
                if neighbor_id.0 < self.id.0 {
                    for x in accessible_thru_neighbors_neighbor {
                        if x.0 < queen_inner {
                            queen_inner = x.0
                        };
                        if x.0 < self.id.0 {
                            flow.insert(x.clone());
                        } else {
                            tree.insert(x.clone());
                        }
                    }
                } else if neighbor_id.0 > self.id.0  {
                    for x in accessible_thru_neighbors_neighbor {
                        if x.0 < queen_inner {
                            queen_inner = x.0
                        };
                        if x.0 > self.id.0 {
                            flow.insert(x.clone());
                        } else {
                            tree.insert(x.clone());
                        }
                    }
                } else {
                    unreachable!("neighbor should be higher or lower than self")
                }
            }
        }
        if let Some(neighbor_info) = self.neighbors.get_mut(neighbor_id) {
            neighbor_info.queen = NodeId(queen_inner);
            neighbor_info.princess = princess.clone();
            neighbor_info.flow = flow;
            neighbor_info.tree = tree;
            neighbor_info.last_seen.insert(radio.clone(), recieved_time);
        } else {
            let seen = BTreeMap::from([(radio.clone(),recieved_time)]);
            let info = NeighborInfo{ 
                queen: NodeId(queen_inner),
                princess: princess.clone(),
                flow, 
                tree, 
                first_seen: seen.clone(), 
                last_seen: seen,
            };
            self.neighbors.insert(neighbor_id.clone(), info);
        }
    }
}