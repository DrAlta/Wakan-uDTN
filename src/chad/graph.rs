use ordered_f32::OrderedF32;
use std::{
    collections::{btree_map::Keys, BTreeMap, HashMap},
    rc::Rc,
};

use crate::wakan::{NodeId, Radio, Time, Transmission, WirelessNode};

pub type Coord = (OrderedF32, OrderedF32);

use super::Node;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Graph<P, N: WirelessNode<P>> {
    nodes: BTreeMap<NodeId, Node<P, N>>,
}

impl<P, N: WirelessNode<P>> Graph<P, N> {
    pub fn tick_node(
        &mut self,
        time: Time,
        recieved_packets: Vec<(Time, Rc<P>, Radio)>,
        node_id: &NodeId,
    ) -> Result<Vec<Transmission<P>>, String> {
        let node = self
            .nodes
            .get_mut(node_id)
            .ok_or(format!("Failed to get node:{node_id} as mutable."))?;
        node.wireless_node.tick(time, recieved_packets)
    }
    // Access a node by its ID
    pub fn get_node(&self, id: &NodeId) -> Option<&Node<P, N>> {
        self.nodes.get(id)
    }

    // Retrieve all nodes
    pub fn all_nodes(&self) -> impl Iterator<Item = &Node<P, N>> {
        self.nodes.iter().map(|(_k, v)| v)
    }
    // Retrieve all nodes with ida
    pub fn get_nodes(&self) -> &BTreeMap<NodeId, Node<P, N>> {
        &self.nodes
    }
    pub fn get_node_ids(&self) -> Keys<'_, NodeId, Node<P, N>> {
        self.nodes.keys()
    }

    pub fn nodes_coord(&self, node_id: &NodeId) -> Option<Coord> {
        let node = self.get_node(node_id)?;
        Some((node.x, node.y))
    }
    pub fn distance_sqrd_to_node(&self, coord: &Coord, node_id: &NodeId) -> Option<OrderedF32> {
        let (x1, y1) = self.nodes_coord(node_id)?;
        let dx = coord.0 - x1;
        let dy = coord.1 - y1;
        Some((dx * dx) + (dy * dy))
    }
    pub fn distance_to_node(&self, coord: &Coord, node_id: &NodeId) -> Option<OrderedF32> {
        Some(self.distance_sqrd_to_node(coord, node_id)?.sqrt())
    }
    // Check if there's a link between two nodes
    pub fn are_connected(&self, from: &NodeId, to: &NodeId) -> bool {
        self.get_node(from)
            .map(|node| node.outbound_links.contains(to))
            .unwrap_or(false)
    }
    pub fn outbound_neighbor_ids(&self, id: &NodeId) -> Option<&Vec<NodeId>> {
        Some(&self.get_node(id)?.outbound_links)
    }
    // Get all
    pub fn outbound_neighbors(&self, id: &NodeId) -> Option<Vec<&Node<P, N>>> {
        Some(
            self.get_node(id)?
                .outbound_links
                .iter()
                .filter_map(|node_id| self.get_node(node_id))
                .collect(),
        )
    }
    pub fn inbound_neighbors(&self, id: &NodeId) -> Option<Vec<&Node<P, N>>> {
        Some(
            self.get_node(id)?
                .inbound_links
                .iter()
                .filter_map(|node_id| self.get_node(node_id))
                .collect(),
        )
    }
}

impl<P, N: WirelessNode<P>> Graph<P, N> {
    pub fn from_raw_nodes(raw_nodes: Vec<super::RawNode>) -> Self {
        let mut nodes_map: HashMap<NodeId, Node<P, N>> = HashMap::new();

        for raw in &raw_nodes {
            nodes_map.insert(
                raw.id,
                Node::new(raw.id, raw.x, raw.y, raw.outbound_links.clone(), Vec::new()),
            );
        }

        for raw in &raw_nodes {
            for target_id in &raw.outbound_links {
                if let Some(target_node) = nodes_map.get_mut(target_id) {
                    target_node.inbound_links.push(raw.id);
                }
            }
        }

        Graph {
            nodes: nodes_map.into_iter().collect(),
        }
    }
    fn to_raw_nodes(&self) -> Vec<super::RawNode> {
        self.nodes.iter().map(|(_, x)| x.into()).collect()
    }

    pub fn to_json_string(&self) -> String {
        let raw_nodes = self.to_raw_nodes();
        serde_jsonrc::to_string_pretty(&raw_nodes).expect("Failed to serialize nodes to JSON")
    }
}
