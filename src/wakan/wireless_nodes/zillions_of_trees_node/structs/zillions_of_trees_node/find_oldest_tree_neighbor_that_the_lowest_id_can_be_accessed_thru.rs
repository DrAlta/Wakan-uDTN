use std::cmp::Ordering;

use crate::wakan::{NodeId, ZillionsOfTreesNode};

impl ZillionsOfTreesNode {
    pub fn find_oldest_tree_neighbor_that_the_lowest_id_can_be_accessed_thru(
        &self,
    ) -> Option<NodeId> {
        let (neighbor_id, _, _) = self
            .tree_neighbors
            .iter()
            .filter_map(|neighbor_id| {
                let neighbor_info = self.neighbors.get(neighbor_id)?;
                Some((
                    neighbor_id,
                    neighbor_info
                        .find_lowest_id_accessible_thru()
                        .unwrap_or(neighbor_id.clone()),
                    neighbor_info.find_oldest_time()?,
                ))
            })
            .min_by(
                |(_, a_lowest, a_age), (_, b_lowest, b_age)| match a_lowest.cmp(b_lowest) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => a_age.cmp(b_age),
                    Ordering::Greater => Ordering::Greater,
                },
            )?;
        Some(neighbor_id.clone())
    }
}
