use ghx_constrained_delaunay::{triangulation::TriangulationConfiguration, triangulation_from_2d_vertices, types::Vertex as Point};
use rand::Rng;

use crate::{chad::{Graph, RawNode}, wakan::{NodeId, WirelessNode}};

pub fn generate_random_graph<P, N: WirelessNode<P>>(
    num_nodes: usize, 
    width: f32, 
    height: f32,
    min_dist: f32,
) -> Graph<P, N> {
    let min_dist_sqrd= min_dist * min_dist;
    let mut rng = rand::rng();

    // Step 1: Create RawNodes with random positions
    let mut raw_nodes = Vec::new();
    // Place nodes with minimum distance constraint
    while raw_nodes.len() < num_nodes {
        let x = rng.random_range(50.0..(width - 50.0));
        let y = rng.random_range(50.0..(height - 50.0));

        let too_close = raw_nodes.iter().any(|node: &RawNode| {
            let dx = node.x - x;
            let dy = node.y - y;
            (dx * dx + dy * dy) < min_dist_sqrd
        });

        if !too_close {
            let id = raw_nodes.len() as NodeId;
            raw_nodes.push(RawNode {
                id,
                x,
                y,
                outbound_links: Vec::new(),
            });
        }
    }

    // Step 2: Run Delaunay triangulation on the node positions
    let vertices: Vec<Point> = raw_nodes
        .iter()
        .map(|node| Point {
            x: node.x as f64,
            y: node.y as f64,
        })
        .collect();

    let triangulation =
        triangulation_from_2d_vertices(&vertices, TriangulationConfiguration::default()).unwrap();

    // Step 3: Extract edges from triangles
    let mut edge_set = std::collections::HashSet::new();
    for indices in triangulation.triangles.iter() {
        for i in 0..3 {
            let a = indices[i] as NodeId;
            let b = indices[(i + 1) % 3] as NodeId;
            // Avoid duplicates and self-links
            if a != b {
                edge_set.insert((a, b));
                edge_set.insert((b, a));
            }
        }
    }

    // Step 4: Assign outbound links
    for &(a, b) in &edge_set {
        raw_nodes[a as usize].outbound_links.push(b);
        raw_nodes[b as usize].outbound_links.push(a); // Make it symmetric if desired
    }

    // Step 5: Build the full graph with inbound links
    Graph::from_raw_nodes(raw_nodes)
}
