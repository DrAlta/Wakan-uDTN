use qol::logy;

use crate::wakan::{wireless_nodes::scoms_tree_node::MAX_AGE, ScomsTreeNode, Time};

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
                logy!("trace-scoms-tree-node", "{:?} removed child {x:?}", self.id,);
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
                    "trace-scoms-tree-node",
                    "{:?}'s' parent, {parent_id:?}, went offline",
                    self.id
                );
            };
            x
        } else {
            true
        };

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
                        "trace-scoms-tree-node",
                        "\n{:?} set its parent to {:?}",
                        self.id,
                        oldest_id
                    );
                    self.parent_maybe = Some(oldest_id.clone());
                } else {
                    logy!(
                        "trace-scoms-tree-node",
                        "\n{:?} failed to find a new parent",
                        self.id,
                    );
                    panic!();
                    self.parent_maybe = None;
                }
            }
        }
    }
}
