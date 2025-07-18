use std::collections::BTreeSet;

use ghx_constrained_delaunay::{
    triangulation::TriangulationConfiguration, triangulation_from_2d_vertices,
    types::Vertex as Point,
};

use crate::wakan::{Graph, WirelessNode};

impl<P, N: WirelessNode<P>> Graph<P, N> {
    pub fn delaunay(&mut self) {
        let mut node_ids = Vec::new();
        let vertices: Vec<_> = self
            .nodes
            .iter()
            .map(|(id, node)| {
                node_ids.push(id.clone());
                Point {
                    x: Into::<f64>::into(node.x),
                    y: Into::<f64>::into(node.y),
                }
            })
            .collect();
        let triangulation =
            triangulation_from_2d_vertices(&vertices, TriangulationConfiguration::default())
                .unwrap();

        // Step 3: Extract edges from triangles
        let mut edge_set = std::collections::HashSet::new();
        for indices in triangulation.triangles.iter() {
            for i in 0..3 {
                let a = node_ids[indices[i] as usize].clone();
                let b = node_ids[indices[(i + 1) % 3] as usize].clone();
                // Avoid duplicates and self-links
                if a != b {
                    edge_set.insert((a.clone(), b.clone()));
                    edge_set.insert((b, a));
                }
            }
        }

        // Step 4: clear all links

        for (_, node) in &mut self.nodes {
            node.outbound_links = BTreeSet::new();
            node.inbound_links = BTreeSet::new();
        }

        // Step 5: Assign links
        for (a_id, b_id) in &edge_set {
            let a = self
                .nodes
                .get_mut(a_id)
                .expect("we go the id by looping over them");
            a.outbound_links.insert(b_id.clone());
            a.inbound_links.insert(b_id.clone());
            let b = self
                .nodes
                .get_mut(b_id)
                .expect("we go the id by looping over them");
            b.outbound_links.insert(a_id.clone());
            b.inbound_links.insert(a_id.clone());
        }
    }
}
