pub use macroquad::prelude::*;

use crate::wakan::{Graph, WirelessNode};

pub fn draw_directed_edges<
    P: Ord + PartialOrd + Eq + PartialEq,
    T: WirelessNode<P> + Ord + PartialOrd + Eq + PartialEq,
>(
    arrow_head_size: f32,
    node_size: f32,
    graph: &Graph<P, T>,
) {
    let draw_at_tail = true;

    // Draw edges
    for node in graph.all_nodes() {
        for link_id in &node.outbound_links {
            if let Some(target) = graph.get_node(link_id) {
                // Perpendicular vector (90° counter-clockwise)
                let head = vec2(node.x.into(), node.y.into());
                let tail = vec2(target.x.into(), target.y.into());
                let line = tail - head;
                let dir = line.normalize();
                let perp = Vec2::new(-dir.y, dir.x);

                let (arrow_tip, offset) = if draw_at_tail {
                    (
                        tail - (dir * node_size),
                        tail - (dir * (arrow_head_size + node_size)),
                    )
                } else {
                    (
                        head + (dir * node_size),
                        head + (dir * (arrow_head_size + node_size)),
                    )
                };
                let arrow_point = offset + (perp * arrow_head_size);
                draw_triangle(arrow_tip, arrow_point, offset, RED);
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
