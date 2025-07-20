use ghx_constrained_delaunay::types::Vertex as Point;

use wakan_sim::wakan::{BeepyNode, BeepyPacket, Frontend, Graph, WakamSim};

#[macroquad::main("Donut Test")]
pub async fn main() {
    let graph: Graph<BeepyPacket, BeepyNode> =
        Graph::generate_donut_graph(3, 2, 75.0, Point::new(50.0, 50.0));
    let mut sim = WakamSim::new(graph);
    //panic!();
    loop {
        sim.tick_sim().await
    }
}
