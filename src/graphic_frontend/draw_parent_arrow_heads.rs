pub use macroquad::prelude::*;

use crate::wakan::{Graph, Parent, WirelessNode};

pub fn draw_parent_arrow_heads<P, T: WirelessNode<P> + Parent>(
    arrow_head_size: f32,
    node_size: f32,
    graph: &Graph<P, T>,
) {
    for node in graph.all_nodes() {
        let draw_at_tail = true;
        if let Some(parent_id) = node.wireless_node.get_parent() {
            if let Some(target) = graph.get_node(&parent_id) {
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
            }
        } else {
            draw_circle(node.x.into(), node.y.into(), 5.0, GREEN);
        }
    }
}
