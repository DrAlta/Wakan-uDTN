use qol::logy;

use super::super::super::{ScomsTreeNode, MAX_AGE};
use crate::wakan::Time;
impl ScomsTreeNode {
    pub fn update(&mut self, now: Time) {
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

        if let Some((neighbor_accessible_by, lowest_id_accessible_by_neighbor)) =
            self.find_lowest_id_accessible_by_neighbor()
        {
            if need_new_parent_ka {
                if self.parent_maybe.is_some() {
                    logy!("info", "\n\n----\n\nresetting {:?}'s parent", self.id);
                };
                // if we found out set it to are new parent
                if lowest_id_accessible_by_neighbor.0 < self.id.0 {
                    logy!(
                        "trace-scoms-tree-node-update",
                        "\n{:?} set its parent to {:?}",
                        self.id,
                        oldest_id
                    );
                    self.parent_maybe = Some(neighbor_accessible_by.clone());
                } else {
                    self.parent_maybe = None;
                }
            } else {
                match (
                    lowest_id_accessible_by_neighbor
                        .0
                        .cmp(&self.lowest_known_node_id.0),
                    lowest_id_accessible_by_neighbor.0 < self.id.0,
                ) {
                    (std::cmp::Ordering::Less, true) => {
                        logy!(
                            "trace-scoms-tree-node-update",
                            "{:?} found a new lowestID",
                            self.id,
                        );
                        let new_parent_maybe = Some(neighbor_accessible_by.clone());
                        // set the new lowest id
                        self.lowest_known_node_id = lowest_id_accessible_by_neighbor.clone();
                        self.parent_maybe = new_parent_maybe;
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
                        self.lowest_known_node_id = lowest_id_accessible_by_neighbor.clone();
                    }
                    (std::cmp::Ordering::Greater, false) => {
                        // lowest id known by neigbors have gone up and isn't lower than my own id
                        self.parent_maybe = None;
                    }
                }
            }
        } else {
            logy!(
                "trace-scoms-tree-node-update",
                "{:?} doesn't have neighbors:\n\n{:?}\n\n",
                self.id,
                self.neighbors
            );
            if need_new_parent_ka {
                logy!(
                    "trace-scoms-tree-node-update",
                    "{:?}: no parent canidates clearing parent",
                    self.id,
                );
                self.parent_maybe = None;
            }
        }

        /*
        // if we need a new parent look for one
        if need_new_parent_ka {
            if self.parent_maybe.is_some() {
                logy!("info", "\n\n----\n\nresetting {:?}'s parent", self.id);
            };
            // find the oldest neighbor we know of the neighbors that have the lowest ID accessable thru them.
            if let Some(oldest_id) =
                self.find_oldest_neighbor_that_the_lowest_id_can_be_accessed_thru()
            {
                // if we found out set it to are new parent
                if oldest_id.0 < self.id.0 {
                    logy!(
                        "trace-scoms-tree-node-update",
                        "\n{:?} set its parent to {:?}",
                        self.id,
                        oldest_id
                    );
                    self.parent_maybe = Some(oldest_id.clone());
                } else {
                    logy!(
                        "trace-scoms-tree-node-update",
                        "\n{:?} failed to find a new parent",
                        self.id,
                    );
                    self.parent_maybe = None;
                }
            }
        }*/
    }
}
