use std::collections::BTreeSet;

use ghx_constrained_delaunay::types::Vertex as Point;
use qol::logy;

use crate::wakan::{Graph, NodeId, RawNode, WirelessNode};

impl<P, N: WirelessNode<P>> Graph<P, N> {
    pub fn generate_donut_graph(
        thinkness: usize,
        gap: usize,
        spacing: f32,
        kerning: Point,
    ) -> Graph<P, N> {
        // Step 1: Create RawNodes with random positions
        let mut raw_nodes = Vec::new();
        // Place nodes with minimum distance constraint
        let mut node_id_counter = 0_usize;
        let side_length = thinkness + gap;
        // crate top(unto the gap)
        for i in 0..thinkness {

            let y = i as f32 * spacing + kerning.y as f32;

            let start_id_of_next_row = {
                let mut acc = side_length;
                for j in 0..i {
                    acc += side_length + j + 1
                }
                acc
            };

            let x_start = ((((side_length + gap) - (i)) as f32 * 0.5) - 0.0) * spacing + kerning.x as f32;

            for j in 0..side_length + i {
                let x = x_start + (j as f32 * spacing);

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;
                let outbound_links = if i < thinkness - 1 {
                    logy!("debug", "i < thinkness-1");
                    BTreeSet::from([
                        NodeId::from(start_id_of_next_row + j),
                        NodeId::from(start_id_of_next_row + j + 1),
                    ])
                } else {
                    if j < thinkness - 1 {
                        logy!("debug", "j < thinkness - 1");

                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j),
                            NodeId::from(start_id_of_next_row + j + 1),
                        ])
                    } else if j > thinkness + gap - 1 {
                        logy!("debug", "j > thinkness + gap");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j - gap),
                            NodeId::from(start_id_of_next_row + j + 1 - gap),
                        ])
                    } else if j == thinkness - 1 {
                        logy!("debug", "j == thinkness -1");
                        BTreeSet::from([NodeId::from(start_id_of_next_row + j)])
                    } else if j >= thinkness && j < thinkness + gap - 1 {
                        logy!("debug", ">= thinkness && j < thinkness + gap - 1");
                        BTreeSet::new()
                    } else if j == thinkness + gap - 1 {
                        logy!("debug", "j == thinkness + gap - 1");
                        BTreeSet::from([NodeId::from(start_id_of_next_row + thinkness)])
                    } else {
                        //BTreeSet::new()
                        unreachable!("j:{j}")
                    }
                };
                println!("node {id} : {outbound_links:?}");
                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
            println!("----");
        }

        //top of gap
        let start_id_of_gap = {
            let mut acc = side_length;
            for j in 0..thinkness - 1 {
                acc += side_length + j + 1
            }
            acc
        };
        for i in 0..gap {
            let start_id_of_next_row = start_id_of_gap + ((thinkness + thinkness) * (i + 1));
            let y = ((thinkness + i) as f32 * spacing) + kerning.y as f32;
            let start = (side_length + gap) - (i + thinkness);
            let x_start = ((start as f32 * 0.5) - 0.0) * spacing;
            for j in 0..thinkness {
                let x = x_start + (j as f32 * spacing) + kerning.x as f32;

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = if i == gap -1 {
                    if j == 0 {
                        logy!("debug", "start of first half");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row),
                            NodeId::from(start_id_of_next_row + 1),
                        ])

                    } else if j < thinkness -1 {
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j+1),
                            NodeId::from(start_id_of_next_row + j),
                        ])
                    } else {
                        logy!("debug", "end of first half");
                        BTreeSet::from([
                            NodeId::from(id.0-1),
                            NodeId::from(start_id_of_next_row + j),
                        ])
                    }
                } else {
                    if j < thinkness - 1 {
                        logy!("debug", "j < thinkness -1 ");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j),
                            NodeId::from(start_id_of_next_row + j + 1),
                        ])
                    } else {
                        BTreeSet::from([NodeId::from(start_id_of_next_row + j)])
                    }
                };
                logy!("debug","node {id} : {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
            for j in 0..thinkness {
                let x = x_start + ((j + thinkness + gap + i) as f32 * spacing);
                //                let x =  (spacing * (j +  thinkness + gap) as f32 )  + kerning.x as f32;

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = if i == gap -1 {
                    if j ==0 {
                        logy!("debug", "the start of thesecond half");
                        BTreeSet::from([
                                NodeId::from(thinkness + start_id_of_next_row),
                            ])                        
                    } else if j > 0 && j < thinkness -1 {
                    logy!("debug", "the middle of the second half");
                        BTreeSet::from([
                            NodeId::from(thinkness + start_id_of_next_row + j),
                            NodeId::from(thinkness + start_id_of_next_row + j - 1),
                        ])
                    } else {
                        logy!("debug", "the end of the second half");
                        BTreeSet::from([
                            NodeId::from(thinkness + start_id_of_next_row + j),
                            NodeId::from(thinkness + start_id_of_next_row + j - 1) ,
                            ])
                    }
                } else {
                    if j == 0 {
                        BTreeSet::from([NodeId::from(start_id_of_next_row + thinkness )]
                    )
                    } else if j == thinkness + thinkness  {
                        BTreeSet::from([
                            NodeId::from(thinkness + start_id_of_next_row + j - 1),
                        ])
                    } else {
                        BTreeSet::from([
                            NodeId::from(thinkness + start_id_of_next_row + j - 1),
                            NodeId::from(thinkness + start_id_of_next_row + j),
                        ])
                    }
                };
                logy!("debug","node {id} : {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
        }


        // gap bottom
        for i in 0..gap + 1 {
            let start_id_of_next_row = start_id_of_gap +((thinkness + thinkness) * (i +1+ gap));
            let y = ((gap + thinkness + i) as f32 * spacing) + kerning.y as f32;
            let x_start = (
                (
                    (
                        (
                            side_length as f32  - (thinkness as f32 - i as f32)
                        ) * 0.5 
                    ) 
                ) * spacing 
            ) + kerning.x as f32;
            for j in 0..thinkness {
                let x = x_start + (j as f32 * spacing);

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = if j == 0 {
                    BTreeSet::from([
                        NodeId::from(start_id_of_next_row ),
                    ])
                } else/* if j < thinkness - 1*/{
                    BTreeSet::from([
                        NodeId::from(start_id_of_next_row + j),
                        NodeId::from(start_id_of_next_row + j - 1)
                    ])
                };
                logy!("debug","node {id} : {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
            for j in 0..thinkness {
                let x = x_start + ((j + thinkness + gap + gap - i ) as f32 * spacing);

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = if j == thinkness - 1{
                    BTreeSet::from([
                        NodeId::from(start_id_of_next_row + thinkness+ j)
                    ])
                } else {
                    BTreeSet::from([
                        NodeId::from(start_id_of_next_row + thinkness + j + 1),
                        NodeId::from(start_id_of_next_row + thinkness + j)
                    ])
                };
                logy!("debug", "node {id} : {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
        }
/*
        //temp
        let y = spacing * (thinkness+gap+gap-1) as f32 + kerning.y as f32;
        for i in 0..thinkness+thinkness+gap -1{
            let x = spacing * ( 2+ i) as f32 + kerning.x as f32;

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = BTreeSet::new();;

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });

        }*/
        /*
                // Step 2: Run Delaunay triangulation on the node positions
                let vertices: Vec<Point> = raw_nodes
                    .iter()
                    .map(|node| Point {
                        x: node.x as f64,
                        y: node.y as f64,
                    })
                    .collect();

                let triangulation =
                    triangulation_from_2d_vertices(&vertices, TriangulationConfiguration::default())
                        .unwrap();

                // Step 3: Extract edges from triangles
                let mut edge_set = std::collections::HashSet::new();
                for indices in triangulation.triangles.iter() {
                    for i in 0..3 {
                        let a = NodeId::from(indices[i]);
                        let b = NodeId::from(indices[(i + 1) % 3]);
                        // Avoid duplicates and self-links
                        if a != b {
                            edge_set.insert((a.clone(), b.clone()));
                            edge_set.insert((b, a));
                        }
                    }
                }

                // Step 4: Assign outbound links
                for (a, b) in &edge_set {
                    raw_nodes[a.0 as usize].outbound_links.insert(b.clone());
                    raw_nodes[b.0 as usize].outbound_links.insert(a.clone()); // Make it symmetric if desired
                }
        */
        // Step 5: Build the full graph with inbound links
        Graph::from_raw_nodes(raw_nodes)
    }
}
