use std::collections::BTreeSet;

use crate::wakan::{wireless_nodes::zillions_of_trees_node::structs::{neighbor_info, ZillionsOfTreesPacket}, RecievedTime, WirelessNode, ZillionsOfTreesNode};

impl WirelessNode<ZillionsOfTreesPacket> for ZillionsOfTreesNode{
    fn tick(
        &mut self,
        now: crate::wakan::Time,
        recieved_packets: Vec<(crate::wakan::RecievedTime, std::rc::Rc<ZillionsOfTreesPacket>, crate::wakan::Radio)>,
    ) -> Result<Vec<crate::wakan::Transmission<ZillionsOfTreesPacket>>, String> {

        for (recieved_time, packet_rc, _radio) in recieved_packets{
            let packet = packet_rc.as_ref();
            match packet{
                ZillionsOfTreesPacket::Beacon { source, neighbors } => {
                    let mut lowlow = BTreeSet::new();
                    let mut lowhigh = BTreeSet::new();
                    let mut highlow = BTreeSet::new();
                    let mut highhigh = BTreeSet::new();
                    for (neighbor_id, accessible) in neighbors {
                        if neighbor_id.0 != self.id.0 {
                            let (low, high) = if source.0 < self.id.0 {
                                (&mut lowlow, &mut lowhigh)
                            } else {
                                (&mut highlow, &mut highhigh)
                            };
                            for x in accessible {
                                if x.0 < self.id.0 {
                                    low.insert(x.clone());
                                } else {
                                    high.insert(x.clone());
                                }
                            }
                        }
                    }

                    /*
                    for (neighbor_id, (lowest_accessible, highest_accessible)) in neighbors {
                        if source.id.0 < self.id.0 {
                            for x in lowest_accessible {
                                low.insert(x.clone());
                            }
                        } else if source.id.0 > self.id.0 {
                            for x in highest_accessible {
                                high.insert(x.clone());
                            }
                        }
                    }
                    */
                },
            };
        }
        todo!()

    }

    fn new(id: crate::wakan::NodeId) -> Self {
        todo!("{}", id)
    }
}