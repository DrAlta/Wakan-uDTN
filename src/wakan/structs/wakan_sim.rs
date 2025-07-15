use std::{collections::HashMap, i32, rc::Rc};

use ordered_f32::OrderedF32;
use qol::logy;

use crate::wakan::{Graph, NodeId, Radio, Time, Transmission, WirelessNode};

type ScheduledReceptionTime = Time;
type ReceiverNodeId = NodeId;
type TransmitterNodeId = NodeId;

const SCALE: f32 = 0.0;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct WakamSim<P, N: WirelessNode<P>> {
    // Represents the simulation's network topology and node states.
    pub graph: Graph<P, N>,
    // Queue of packets that are scheduled to be received by a node at a specific time.
    pub scheduled_receptions: Vec<(
        TransmitterNodeId,
        ReceiverNodeId,
        ScheduledReceptionTime,
        Rc<P>,
        Radio,
    )>,
}
impl<
        P: std::fmt::Debug + Ord + PartialOrd + Eq + PartialEq,
        N: WirelessNode<P> + Ord + PartialOrd + Eq + PartialEq,
    > WakamSim<P, N>
{
    /// This function represents a single simulation step at the given time.
    pub fn tick(&mut self, time: Time) -> Option<Time> {
        let WakamSim {
            graph,
            scheduled_receptions,
        } = self;

        // Create an empty queues of packets arriving for each node.
        let mut queues: HashMap<NodeId, Vec<(Time, Rc<P>, Radio)>> = graph
            .get_node_ids()
            .map(|node_id| (node_id.clone(), Vec::new()))
            .collect();
        let mut next_reception = None;
        let mut count_of_transmissions = 0;
        let _ = &count_of_transmissions; // to get read of the lints about it being unused as it's only used currently by the feature that isn't a default feature
                                         // Process up to 100 receptions per tick.
        let limit = i32::MAX;
        for i in 0..limit {
            if i == limit - 1 {
                logy!(
                    "trace-wakan-sim",
                    "hit loop limit. {:?} receptions still scheduled.",
                    scheduled_receptions.len()
                );
            };
            // Sort receptions by scheduled time, in descending order (to pop earliest last).
            scheduled_receptions.sort_by(
                |(_a_transmitter, _a_receiver, a_time, _a_packet, _a_radio),
                 (_b_transmitter, _b_receiver, b_time, _b_packet, _b_radio)| {
                    b_time.cmp(a_time)
                },
            );

            // Peek at the last item (earliest time due to sort order).
            let Some((_transmitter, _receiver, recieved_time, _packet, _radio)) =
                scheduled_receptions.last()
            else {
                logy!(
                    "trace-wakan-sim",
                    "No packets left exiting reception sorting loop."
                );
                break;
            };

            if next_reception.is_none() {
                next_reception = Some(*recieved_time);
            }

            if recieved_time > &time {
                logy!(
                    "trace-wakan-sim",
                    "finished sorting all receptions upto now{time}, next:{next_reception:?}: total:{}",
                    scheduled_receptions.len()
                );
                break;
            };

            // Pop the reception and add it to the receiver's queue.
            let (_transmitter, receiver, recieved_time, shared_packet, radio) =
                scheduled_receptions
                    .pop()
                    .expect("we checked that is had an item so it shoulf still be there");
            let Some(queue) = queues.get_mut(&receiver) else {
                logy!(
                    "error",
                    "there is no node with id:{receiver}, next:{next_reception:?}"
                );
                continue;
            };
            queue.push((recieved_time, shared_packet, radio));
        }
        logy!("trace-wakan-sim", "now: {time}, next:{next_reception:?}");

        // Iterate over each node that had packets delivered.
        'receivers: for (receiver, recieved_packets) in queues.into_iter() {
            let count_of_received_packets = recieved_packets.len();

            // Simulate this node’s logic for processing received packets.
            let new_transmissions = match graph.tick_node(time, recieved_packets, &receiver) {
                Ok(ok) => {
                    /*if (!ok.is_empty()) || count_of_received_packets != 0 {
                        logy!(
                            "info",
                            "node:{receiver} recieved {count_of_received_packets} and sent {}",
                            ok.len()
                        );
                    };*/
                    ok
                }
                Err(err) => {
                    logy!(
                        "info",
                        "{count_of_received_packets} packets where send to node:{receiver} but it failed with err:{err:?}",
                    );
                    continue;
                }
            };
            // add the number of transmissions to the count
            count_of_transmissions += new_transmissions.len();

            let Some(coord) = graph.nodes_coord(&receiver) else {
                logy!("error", "could get coord on node{receiver}");
                continue;
            };
            for Transmission {
                when: scheduled_transmission_time,
                packet,
                radio: _,
            } in new_transmissions
            {
                let Some(neighbor_ids) = graph.outbound_neighbor_ids(&receiver) else {
                    logy!("error", "could get outbound neighbors of {receiver}");
                    continue 'receivers;
                };

                let shared_packet = Rc::new(packet);
                for neighbor_id in neighbor_ids {
                    if scheduled_transmission_time < time {
                        logy!("info", "{shared_packet:?}");
                    };
                    let recieved_time = scheduled_transmission_time
                        + Into::<u64>::into(
                            graph
                                .distance_to_node(&coord, neighbor_id)
                                .unwrap_or(OrderedF32::ONE)
                                * SCALE,
                        );
                    scheduled_receptions.push((
                        receiver.clone(),
                        neighbor_id.clone(),
                        recieved_time,
                        shared_packet.clone(),
                        0.into(),
                    ))
                }
            }
        } // end looping over receptions
        logy!("trace-wakan-sim", "{count_of_transmissions} transmissions");
        next_reception
    }
    pub fn new(graph: Graph<P, N>) -> Self {
        Self {
            graph,
            scheduled_receptions: Vec::new(),
        }
    }
    pub fn get_graph(&self) -> &Graph<P, N> {
        &self.graph
    }
}
