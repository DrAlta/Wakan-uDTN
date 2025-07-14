use crate::wakan::NodeId;

type NeighborsNeighborId = NodeId;
type LowestId = NodeId;
pub fn find_lowest_id_lowest_accessable_thru_neighbor<'a, I: Iterator<Item=(&'a NeighborsNeighborId, &'a LowestId)>>(
    iter: I, 
    neighbor_id: &NodeId, 
    self_id: &NodeId,
) -> NodeId{
    NodeId(
        match iter.filter_map(
            |(neighbors_neighbor, lowest)|
            {
                if neighbors_neighbor != self_id {
                    Some(lowest.0)
                } else {
                    None
                }
            }
        ).min(){
            Some(x) => x.min(neighbor_id.0),
            None => neighbor_id.0,
        }
    )
}