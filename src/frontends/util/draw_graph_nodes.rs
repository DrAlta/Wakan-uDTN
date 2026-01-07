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
        let text_size = 16.0;
        draw_circle(node.x.into(), node.y.into(), node_size, BLUE);
        draw_text(
            &format!("{}", node.id.0),
            Into::<f32>::into(node.x) - node_size + (text_size / 6.0),
            Into::<f32>::into(node.y) + (node_size * 0.5) - (text_size / 12.0),
            text_size,
            BLACK,
        );
    }
}
