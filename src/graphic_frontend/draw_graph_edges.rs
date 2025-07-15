pub use macroquad::prelude::*;

use crate::wakan::{Graph, WirelessNode};

pub fn draw_graph_edges<
    P: Ord + PartialOrd + Eq + PartialEq,
    N: WirelessNode<P> + Ord + PartialOrd + Eq + PartialEq,
>(
    graph: &Graph<P, N>,
) {
    // Draw edges
    for node in graph.all_nodes() {
        for link_id in &node.outbound_links {
            if let Some(target) = graph.get_node(link_id) {
                draw_line(
                    node.x.into(),
                    node.y.into(),
                    target.x.into(),
                    target.y.into(),
                    2.0,
                    GRAY,
                );
            }
        }
    }
}
