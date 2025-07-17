pub use macroquad::prelude::*;

use crate::wakan::{Graph, WirelessNode};

pub fn draw_graph_nodes<
    P: Ord + PartialOrd + Eq + PartialEq,
    N: WirelessNode<P> + Ord + PartialOrd + Eq + PartialEq,
>(
    node_size: f32,
    graph: &Graph<P, N>,
) {
    for node in graph.all_nodes() {
        draw_circle(node.x.into(), node.y.into(), node_size, BLUE);
        draw_text(
            &format!("{}", node.id),
            Into::<f32>::into(node.x) + 12.0,
            node.y.into(),
            16.0,
            BLACK,
        );
    }
}
