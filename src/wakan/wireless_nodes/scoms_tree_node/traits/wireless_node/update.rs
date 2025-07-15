use qol::logy;

use crate::wakan::{
    wireless_nodes::scoms_tree_node::MAX_AGE, ScomsTreeNode, ScomsTreePacket, Time, Transmission,
};

impl ScomsTreeNode {
    pub fn update(
        &mut self,
        mut merge_trees_ka: bool, // `merge_trees_ka` is part of the delay to give time for the lowest id in tree mergers to travel along the tree
        heard_new_root_from_announcment_ka: bool,
        transmissions: &mut Vec<Transmission<ScomsTreePacket>>,
        now: Time,
    ) {
        // cutoff is the limit for consider nodes offline if we haven't heard from them
        // on or after cutoff then we cosider they have gone offline
        let cutoff = now.saturating_sub(MAX_AGE);

        // neighbors_that_went_down will hold the ids of neighbors that went down
        let mut ids_neighbors_that_went_down = Vec::new();
        // we will only retain the naighbors that on online still
        self.neighbors.retain(|neighbor_id, neighbor_info| {
            // remove all times we heard them that was before the cutoff
            neighbor_info.last_seen.retain(|_, time| *time >= cutoff);
            // if there is no last_seen times left then they are offline
            if neighbor_info.last_seen.is_empty() {
                // add theri id to list of neighbors that when down
                ids_neighbors_that_went_down.push(neighbor_id.clone());
                // return false so that it won't be retained
                false
            } else {
                // we had a last_seen left over after filtering out all the
                // ones before the cutoff, so return true to retain it
                true
            }
        });

        // remove children that went down
        self.children.retain(|x| {
            if ids_neighbors_that_went_down.contains(x) {
                logy!(
                    "trace-scoms-tree-node-update",
                    "{:?} removed child {x:?}",
                    self.id,
                );
                false
            } else {
                true
            }
        });

        if let Some((neighbor_known_by, lowest_id_known_by_neighbor)) =
            self.find_lowest_id_known_by_neighbor()
        {
            match (
                lowest_id_known_by_neighbor
                    .0
                    .cmp(&self.lowest_known_node_id.0),
                lowest_id_known_by_neighbor.0 < self.id.0,
            ) {
                (std::cmp::Ordering::Less, true) => {
                    /*
                    logy!(
                        "trace-scoms-tree-node-update",
                        "{:?} found a new lowestID",
                        self.id,
                    );
                    logy!(
                        "trace-scoms-tree-node-update-tree-merge",
                        "{}: seting lowest_known_dirty to true",
                        self.id
                    );
                    */
                    self.lowest_known_dirty = true;
                    // set the new lowest id
                    self.lowest_known_node_id = lowest_id_known_by_neighbor.clone();
                    if self.parent_maybe.is_none() {
                        merge_trees_ka = true;
                    }
                }
                (std::cmp::Ordering::Less, false) => {
                    // lowest known id is less than old but not lower than my own id
                    // this shouldn't be possable as lowest known should never be hight
                    // than our own id so if its lower that the was it should be lower than our id
                    unreachable!("{:?}", {
                        self.parent_maybe = None;
                    })
                }
                (std::cmp::Ordering::Equal, true) | (std::cmp::Ordering::Equal, false) => {
                    // lowest_known id hasn't changed
                    /*
                    logy!(
                        "trace-scoms-tree-node-update",
                        "{:?} lowest_id remained the same",
                        self.id,
                    );*/
                }
                (std::cmp::Ordering::Greater, true) => {
                    // known lowest ID known by neighbers grow but is still smaller than our own id so set it as the lowest id we known
                    logy!(
                        "trace-scoms-tree-node-update",
                        "{:?} found a new lowestID",
                        self.id,
                    );
                    // set the new lowest id
                    self.lowest_known_node_id = lowest_id_known_by_neighbor.clone();
                }
                (std::cmp::Ordering::Greater, false) => {
                    // lowest id known by neigbors have gone up and isn't lower than my own id
                    logy!(
                        "trace-scoms-tree-node-update-tree-merge",
                        "{:?} is setting it's parent to new lowestID",
                        self.id,
                    );
                    self.parent_maybe = None;
                }
            }

            if merge_trees_ka {
                logy!(
                    "trace-scoms-tree-node-update-tree-merge",
                    "{:?} is setting it's parent to new lowestID",
                    self.id,
                );

                self.lowest_known_dirty = false;
                let new_parent_maybe = Some(neighbor_known_by);
                // set the new lowest id
                self.lowest_known_node_id = lowest_id_known_by_neighbor.clone();
                logy!(
                    "info",
                    "{}: set its parent to {new_parent_maybe:?}",
                    self.id
                );
                self.parent_maybe = new_parent_maybe;
            }
        } else {
            /*
            logy!(
                "trace-scoms-tree-node-update",
                "{:?} doesn't have neighbors:\n\n{:?}\n\n",
                self.id,
                self.neighbors
            );
            */
        }

        // update parent if needed
        // find out if we need a new_parent?
        let need_new_parent_ka = if let Some(parent_id) = &self.parent_maybe {
            let x = ids_neighbors_that_went_down.contains(parent_id);
            if x {
                logy!(
                    "trace-scoms-tree-node-update",
                    "{:?}'s' parent, {parent_id:?}, went offline",
                    self.id
                );
            };
            x
        } else {
            true
        };

        // if we need a new parent look for one
        if need_new_parent_ka || heard_new_root_from_announcment_ka {
            //logy!("debug", "\n{}:is this working\nneed_new_parent_ka:{need_new_parent_ka}\nheard_new_root_from_announcment_ka:{heard_new_root_from_announcment_ka}", self.id);
            if self.parent_maybe.is_some() {
                logy!("info", "resetting {:?}'s parent", self.id);
            };
            // find the oldest neighbor we know of the neighbors that have the lowest ID accessible thru them.
            if let Some(oldest_id) =
                self.find_oldest_neighbor_that_the_lowest_id_can_be_accessed_thru()
            {
                // if we found out set it to are new parent
                if oldest_id.0 < self.id.0 {
                    logy!(
                        "trace-scoms-tree-node-update",
                        "{}: set its parent to {:?}",
                        self.id,
                        oldest_id
                    );
                    self.parent_maybe = Some(oldest_id.clone());

                    logy!(
                        "trace-scoms-tree-node-update-tree-merge",
                        "{}: seting lowest_known_dirty to false",
                        self.id
                    );
                    self.lowest_known_dirty = false;
                    if heard_new_root_from_announcment_ka {
                        logy!(
                            "trace-scoms-tree-node-update-tree-merge",
                            "{}: anouncing tree merger",
                            self.id
                        );
                        let packet = ScomsTreePacket::TreeMerge {
                            source: self.id.clone(),
                            packet_id: {
                                let x = self.send_packet_count.clone();
                                self.send_packet_count += 1;
                                x
                            },
                            new_root: oldest_id.clone(),
                        };
                        logy!("info", "{}: sending Treemerge", self.id);
                        let transmittion = Transmission::new(now, packet, 0.into());
                        transmissions.push(transmittion);
                    }
                } else {
                    logy!(
                        "trace-scoms-tree-node-update",
                        "{}: set its parent to None; failed to find a new parent",
                        self.id,
                    );
                    self.parent_maybe = None;
                }
            }
        }
    }
}
