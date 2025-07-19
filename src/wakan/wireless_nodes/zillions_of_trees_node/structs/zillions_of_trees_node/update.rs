use std::collections::BTreeSet;

use qol::logy;

use crate::wakan::{NodeId, Time};

use super::super::super::{MAX_AGE, ZillionsOfTreesNode};

impl ZillionsOfTreesNode{
    pub fn update(
        &mut self,
        time: Time,
    ) {
        let mut gone_down = BTreeSet::new();
        let cutoff = time - MAX_AGE;
        self.neighbors.retain(
            |id, info|
            {
                if let Some((_, last_seen)) = info.find_last_seen() {
                    if last_seen < &cutoff {
                        gone_down.insert(id.clone());
                        false
                    } else {
                        true
                    }
                } else {
                    gone_down.insert(id.clone());
                    false
                }
            }
        );

        let mut lower = BTreeSet::<NodeId>::new();
        let mut higher = BTreeSet::<NodeId>::new();
         self.neighbors.iter().for_each(
            |(neighbor_id, info)|
            {
                if neighbor_id.0 < self.id.0 {
                    for nn in &info.flow {
                        lower.insert(nn.clone());
                    }
                } else if neighbor_id.0 > self.id.0{
                    for nn in &info.flow {
                        higher.insert(nn.clone());
                    }
                } else {
                    unreachable!("neighbor should be higher or lower than self")
                }
            }
        );

        for (_neighbor_id, info) in &mut self.neighbors {
            info.tree.retain(
                |n| 
                !(lower.contains(n) || higher.contains(n))
            );
        }

        // update who are tree neighbors are, these are neights tht we use to access ndes throu the tree
        self.tree_neighbors = self.neighbors.iter().filter_map(
            |(id, info)|
            {
                if info.tree.is_empty(){
                    None
                } else {
                    Some(id.clone())
                }
            }
        ).collect();

        // update who the priness of are cluster is
        let mut princess_inner = self.id.0;
        for tree_neighbor_id in &self.tree_neighbors {
            let Some(tree_neighbor_info) = self.neighbors.get(tree_neighbor_id) else {
                continue
            };

            if tree_neighbor_info.princess.0 < princess_inner{
                princess_inner = tree_neighbor_info.princess.0;
            }
        }
        let heard_new_princess_from_announcment_ka = if princess_inner != self.princess.0 {
            self.princess = NodeId(princess_inner);
            true
        } else {
            false
        };

        // update are tree parent
        // do we need to find a new parent
        let current_parent_invalid = if let Some(parent) = &self.parent_maybe {
            gone_down.contains(parent) || ! self.tree_neighbors.contains(parent)
        } else {
            true
        };

        if heard_new_princess_from_announcment_ka || current_parent_invalid {
            if let Some(oldest_id) =
                self.find_oldest_tree_neighbor_that_the_lowest_id_can_be_accessed_thru()
            {
                // if we found out set it to are new parent
                if oldest_id.0 < self.id.0 {
                    logy!(
                        "zillions-of-trees-node-update",
                        "{}: set its parent to {:?}",
                        self.id,
                        oldest_id
                    );
                    self.parent_maybe = Some(oldest_id.clone());
                } else {
                    logy!(
                        "zillions-of-trees-node-update",
                        "{}: set its parent to None; failed to find a new parent",
                        self.id,
                    );
                    self.parent_maybe = None;
                }
            }
        }
    }
}