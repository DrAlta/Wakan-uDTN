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
        let y_spacing = spacing * 0.86602540378;
        // Step 1: Create RawNodes with random positions
        let mut raw_nodes = Vec::new();
        // Place nodes with minimum distance constraint
        let mut node_id_counter = 0_usize;
        let side_length = thinkness + gap;
        // crate top(unto the gap)
        for i in 0..thinkness {

            let y = i as f32 * y_spacing + kerning.y as f32;

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
                    logy!("trace-donut-links", "i < thinkness-1");
                    if j < side_length + i -1 {
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j),
                            NodeId::from(start_id_of_next_row + j + 1),
                            NodeId::from(id.0+1),
                        ])
                    } else {
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j),
                            NodeId::from(start_id_of_next_row + j + 1),
                        ])
                    }
                } else {
                    if j < thinkness - 1 {
                        logy!("trace-donut-links", "j < thinkness - 1");

                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j),
                            NodeId::from(start_id_of_next_row + j + 1),
                            NodeId::from(id.0 + 1),
                        ])
                    } else if j == thinkness - 1 {
                        logy!("trace-donut-links", "j == thinkness -1");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j),
                            NodeId::from(id.0 + 1),
                        ])
                    } else if j >= thinkness && j < thinkness + gap - 1 {
                        logy!("trace-donut-links", "{id}: >= thinkness && j < thinkness + gap - 1");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                        ])
                    } else if j == thinkness + gap - 1 {
                        logy!("trace-donut-links", "{id}: j == thinkness + gap - 1");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row + thinkness),
                        ])
                    } else if j > thinkness + gap - 1 && j < side_length + i - 1 {
                        logy!("trace-donut-links", "{id}: j > thinkness + gap - 1");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row - gap + j ),
                            NodeId::from(start_id_of_next_row - gap + 1 + j),
                        ])
                    } else {
                         logy!("trace-donut-links", "{id}: else");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row - gap + j),
                            NodeId::from(start_id_of_next_row + 1 - gap + j),
                        ])
                   }
                };
                logy!("trace-donut-links", "{id}: {outbound_links:?}");
                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
        }
        println!("top finished");

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
            let y = ((thinkness + i) as f32 * y_spacing) + kerning.y as f32;
            let start = (side_length + gap) - (i + thinkness);
            let x_start = ((start as f32 * 0.5) - 0.0) * spacing;
            for j in 0..thinkness {
                let x = x_start + (j as f32 * spacing) + kerning.x as f32;

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = if i == gap -1 {
                    // last row of top of gap
                    if j == 0 {
                        logy!("trace-donut-links", "{id}: start of first half");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row),
                            NodeId::from(start_id_of_next_row + 1),
                        ])

                    } else if j < thinkness -1 {
                        logy!("trace-donut-links", "{id}: j < thinkness -1");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row + j + 1),
                            NodeId::from(start_id_of_next_row + j),
                        ])
                    } else {
                        logy!("trace-donut-links", "{id}: end of first half");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j),
                        ])
                    }
                } else {
                    if j < thinkness - 1 {
                        logy!("trace-donut-links", "{id}: j < thinkness - 1");
                        BTreeSet::from([
                            NodeId::from(id.0  + 1),
                            NodeId::from(start_id_of_next_row + j),
                            NodeId::from(start_id_of_next_row + 1 + j),
                        ])
                    } else {
                        logy!("trace-donut-links", "{id}: else");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j)
                        ])
                    }
                };
                logy!("trace-donut-links", "{id}: {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
            for j in 0..thinkness {
                let x = x_start + ((j + thinkness + gap + i) as f32 * spacing);

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = if i == gap -1 {
                    if j == 0 {
                        logy!("trace-donut-links", "{id}: the start of the second half");
                        BTreeSet::from([
                                NodeId::from(id.0 + 1),
                                NodeId::from(thinkness + start_id_of_next_row),
                            ])                        
                    } else if j > 0 && j < thinkness -1 {
                    logy!("trace-donut-links", "{id}: the middle of the second half");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(thinkness + start_id_of_next_row + j),
                            NodeId::from(thinkness + start_id_of_next_row - 1 + j),
                        ])
                    } else {
                        logy!("trace-donut-links", "{id}:the end of the second half");
                        BTreeSet::from([
                            NodeId::from(thinkness + start_id_of_next_row + j),
                            NodeId::from(thinkness + start_id_of_next_row - 1 + j) ,
                        ])
                    }
                } else {
                    if j == 0 {
                        logy!("trace-donut-links", "{id}: j == 0");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row + thinkness ),
                        ])
                    } else if j == thinkness + thinkness {
                        logy!("trace-donut-links", "{id}: j == thinkness + thinkness");
                        BTreeSet::from([
                            NodeId::from(thinkness + start_id_of_next_row - 1 + j),
                        ])
                    } else  if j < thinkness - 1 {
                        logy!("trace-donut-links", "{id}: j < thinkness -1");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(thinkness + start_id_of_next_row - 1 + j),
                            NodeId::from(thinkness + start_id_of_next_row + j),
                        ])
                    } else{
                        logy!("trace-donut-links", "{id}: else");
                        BTreeSet::from([
                            NodeId::from(thinkness + start_id_of_next_row - 1 + j),
                            NodeId::from(thinkness + start_id_of_next_row + j),
                        ])
                    }
                };
                logy!("trace-donut-links","{id} : {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
        }
        logy!{"debug", "end gap top"}

        // gap bottom
        for i in 0..gap + 1 {
            let start_id_of_next_row = start_id_of_gap +((thinkness + thinkness) * (i +1+ gap));
            let y = ((gap + thinkness + i) as f32 * y_spacing) + kerning.y as f32;
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
                    logy!("trace-donut-links","{id}: j == 0");
                    BTreeSet::from([
                        NodeId::from(id.0 + 1),
                        NodeId::from(start_id_of_next_row ),
                    ])
                } else if j < thinkness - 1{
                    logy!("trace-donut-links","{id}: j < thinkness - 1");
                    BTreeSet::from([
                        NodeId::from(id.0 + 1),
                        NodeId::from(start_id_of_next_row + j),
                        NodeId::from(start_id_of_next_row + j - 1)
                    ])
                } else/* if j < thinkness - 1*/{
                    logy!("trace-donut-links","{id}: else");
                    BTreeSet::from([
                        NodeId::from(start_id_of_next_row + j),
                        NodeId::from(start_id_of_next_row + j - 1)
                    ])
                };
                logy!("trace-donut-links","{id} : {outbound_links:?}");

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

                let outbound_links = if i == thinkness  {
                    if j < thinkness - 1 {
                        logy!("trace-donut-links","{id}: j < thinkness - 1");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row + thinkness + gap + j - 1),
                            NodeId::from(start_id_of_next_row + thinkness + gap + j)
                        ])
                    } else{
                        logy!("trace-donut-links","{id}: else");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + thinkness + gap + j - 1),
                        ])
                    }
                }else{
                    if j == thinkness - 1{
                        logy!("trace-donut-links", "j == thinkness - 1");
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + thinkness+ j)
                        ])
                    } else {
                        logy!("trace-donut-links", "else");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row + thinkness + j + 1),
                            NodeId::from(start_id_of_next_row + thinkness + j)
                        ])
                    }
                };
                logy!("trace-donut-links", "{id} : {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
        }
        logy!("debug", "start bottom");

        // bottom
        let mut start_id_of_next_row = start_id_of_gap + ((thinkness + thinkness) * ( gap + gap +1));

        for i in 0..thinkness {
            let length_of_this_row = side_length + (thinkness - i) - 1;
            start_id_of_next_row += length_of_this_row;
            let y = ((gap + thinkness + gap + 1 + i) as f32 * y_spacing) + kerning.y as f32;


            let x_start = ((((side_length + gap) - (thinkness - i) +1) as f32 * 0.5) - 0.0) * spacing + kerning.x as f32;

            for j in 0..length_of_this_row {
                let x = x_start + (j as f32 * spacing);

                let id = NodeId::from(node_id_counter);
                node_id_counter += 1;

                let outbound_links = if i == thinkness - 1 {
                    logy!("trace-donut-links", "{id}: i == thinkness - 1");
                    if j < length_of_this_row - 1{
                        logy!("trace-donut-links", "{id}: j < length_of_this_row");
                        BTreeSet::from([
                             NodeId::from(id.0 + 1),
                        ])
                    } else{ 
                        logy!("trace-donut-links", "{id}: j else");
                        BTreeSet::new()
                    }
                } else {
                    if j == 0 {
                        logy!("trace-donut-links", "{id}: j == 0");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row ),
                        ])
                    } else if j < length_of_this_row - 1 {
                        logy!("trace-donut-links", "{id}: j < length_of_this_row - 1");
                        BTreeSet::from([
                            NodeId::from(id.0 + 1),
                            NodeId::from(start_id_of_next_row + j - 1),
                            NodeId::from(start_id_of_next_row + j),
                        ])
                    } else{
                        BTreeSet::from([
                            NodeId::from(start_id_of_next_row + j - 1),
                        ])

                    }
                };
                logy!("trace-donut-links", "{id}: {outbound_links:?}");

                raw_nodes.push(RawNode {
                    id,
                    x,
                    y,
                    outbound_links,
                });
            }
        }

        Graph::from_raw_nodes(raw_nodes)
    }
}
