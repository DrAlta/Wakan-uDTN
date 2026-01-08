/*


/// Node structure
#[derive(Clone, Debug)]
pub struct Node {
    _my_address: NodeAddress,
    neighbors: HashSet<NodeAddress>,
    root: [Block; ROOT_SIZE],
    levels: [[(StartOfAddressRange, Block); LEVEL_BUCKETS]; LEVELS],
    canopy: StartOfAddressRange,
    tracked_nodes: HashMap<NodeAddress, Path>,
}

impl Node {

    pub fn add_neighbor(&mut self, n: NodeAddress) {
        self.neighbors.insert(n);
    }

    pub fn add_root_owner(&mut self, idx: usize, owner: NodeAddress) {
        if idx < ROOT_SIZE {
            self.root[idx].add_owner(owner);
        }
    }

    pub fn add_level_owner(&mut self, level: usize, idx: usize, start: StartOfAddressRange, owner: NodeAddress) {
        if level < LEVELS && idx < LEVEL_BUCKETS {
            self.levels[level][idx].0 = start;
            self.levels[level][idx].1.add_owner(owner);
        }
    }

    pub fn add_tracked_node(&mut self, node: NodeAddress, path: Path) {
        self.tracked_nodes.insert(node, path);
    }

*/

use wakan_sim::wakan::DistributedDict;

fn main() {
    let node = DistributedDict::new(0x0100_0000);

    // Query
    if let Some(path) = node.find_next_path(0x0300_0000_0000_0003) {
        let first_hop = path.first_hop();
        println!("Next hop toward target: {first_hop:?}",);
        /*
        if let Some(hop) = first_hop {

            if node.neighbors.contains(&hop) {
                 println!("which is a neighbor");
            } else {
                 println!("which is not a neighbor");
            }
        }
        */
    } else {
        println!("No path known or node offline in canopy");
    }
}
