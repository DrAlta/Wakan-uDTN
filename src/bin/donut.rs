use ghx_constrained_delaunay::types::Vertex as Point;

use wakan_sim::wakan::{BeepyNode, BeepyPacket, Frontend, Graph, WakamSim};

#[macroquad::main("Donut Test")]
pub async fn main() {
    let graph: Graph<BeepyPacket, BeepyNode> =
        Graph::generate_donut_graph(3, 3, 45.0, Point::new(0.0, 10.0));
    let mut sim = WakamSim::new(graph);
    //panic!();
    loop {
        sim.tick_sim().await
    }
}
