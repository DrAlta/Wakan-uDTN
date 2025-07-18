use rand::Rng;

use crate::{
    wakan::{Graph, WirelessNode},
    Number,
};

impl<P, N: WirelessNode<P>> Graph<P, N> {
    pub fn jiggle<R: Rng>(
        &mut self,
        max_offset: Number,
        dom: u8,
        min_x: Number,
        min_y: Number,
        max_x: Number,
        max_y: Number,
        rng: &mut R,
    ) {
        for (_node_id, node) in &mut self.nodes {
            if rng.random::<u8>() < dom {
                let new_x = &node.x + (max_offset * rng.random::<f32>());
                node.x = if new_x < min_x {
                    min_x - (new_x - min_x)
                } else if new_x > max_x {
                    max_x - (new_x - max_x)
                } else {
                    new_x
                };

                let new_y = &node.y + (max_offset * rng.random::<f32>());
                node.y = if new_y < min_y {
                    min_y - (new_y - min_y)
                } else if new_y > max_y {
                    max_y - (new_y - max_y)
                } else {
                    new_y
                };
            }
        }
        self.delaunay();
    }
}
