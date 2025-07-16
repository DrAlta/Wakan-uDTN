use crate::wakan::NodeId;

type NeighborsNeighborId = NodeId;
type LowestId = NodeId;
pub fn find_lowest_id_lowest_accessible_thru_neighbor<
    'a,
    I: Iterator<Item = (&'a NeighborsNeighborId, &'a LowestId)>,
>(
    iter: I,
    neighbor_id: &NodeId,
    self_id: &NodeId,
) -> (NodeId, NodeId) {
    let mut known = None;
    let x = NodeId(
        match iter
            .filter_map(|(neighbors_neighbor, lowest)| {
                if neighbors_neighbor != self_id {
                    Some(lowest.0)
                } else {
                    known = Some(lowest.0.clone());
                    None
                }
            })
            .min()
        {
            Some(x) => x.min(neighbor_id.0),
            None => neighbor_id.0,
        },
    );
    let known = known.unwrap_or(x.0);
    (x, NodeId(known))
}
