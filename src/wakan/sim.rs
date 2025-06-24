use std::rc::Rc;

use ordered_f32::OrderedF32;
use qol::logy;

use crate::{chad::Graph, wakan::{NodeId, Radio, Time, WirelessNode}};

type ScheduledReceptionTime = Time;
type ReceiverNodeId = NodeId;
type TransmitterNodeId = NodeId;

const SCALE: f32 = 1.0;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct WakamSim<P, N: WirelessNode<P>>{
    graph: Graph<P, N>,
    scheduled_receptions: Vec::<(
        TransmitterNodeId,
        ReceiverNodeId,
        ScheduledReceptionTime, 
        Rc<P>, 
        Radio,
    )>
}
impl<P, N: WirelessNode<P>> WakamSim<P, N>{
    pub fn tick(&mut self, time:Time) {
        let WakamSim { graph, scheduled_receptions } = self;
        
        'receptions: for _ in 0..100 {
            scheduled_receptions.sort_by(|
                (_a_transmitter, _a_receiver,a_time, _a_packet, _a_radio), 
                (_b_transmitter, _b_receiver,b_time, _b_packet, _b_radio)
                |{
                    b_time.cmp(a_time)
                }
            );
            let Some((_transmitter, _receiver, recieved_time, _packet, _radio)) = scheduled_receptions.last() else{
                return
            };
            if recieved_time > &time {
                return
            };
            let (_transmitter, receiver, recieved_time, shared_packet, radio) = scheduled_receptions.pop().expect("we checked that is had an item so it shoulf still be there");
            let packet = shared_packet.as_ref();
            let Some(new_transmittions) = graph.tick_node(time, recieved_time, packet, radio, &receiver) else {
                continue;
            };
            let Some(coord) = graph.nodes_coord(&receiver) else {
                logy!("error", "could get coord on node{receiver}");
                continue;
            };
            for ( scheduled_transmition_time, packet, radio ) in new_transmittions {
                let Some(neighbor_ids) = graph.outbound_neighbor_ids(&receiver) else {
                    continue 'receptions
                };
                let shared_packet = Rc::new(packet);
                for neighbor_id in neighbor_ids {
                    let recieved_time = scheduled_transmition_time + Into::<u64>::into(graph.distance_to_node(&coord, neighbor_id).unwrap_or(OrderedF32::ONE) * SCALE);
                    scheduled_receptions.push((receiver, *neighbor_id, recieved_time, shared_packet.clone(), 0))
                }
            }
        }// end looping over receptions
    }
}