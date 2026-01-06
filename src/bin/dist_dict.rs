use std::{array, collections::{HashMap, HashSet}};

type NodeAddress = u64;
type StartOfAddressRange = u64;

const CROWN_SIZE: u64 = 128;

const ROOT_SIZE: usize = 256;
const LEVELS: usize = 7;
const LEVEL_BUCKETS: usize = 128;

/// Path is the intra-block chain of nodes that keep track of a block.
/// The first element (if any) is the next hop from this node.
#[derive(Clone, Debug, Default)]
pub struct Path {
    hops: Vec<NodeAddress>,
}

impl Path {
    fn new() -> Self {
        Self { hops: Vec::new() }
    }

    fn push(&mut self, addr: NodeAddress) {
        self.hops.push(addr);
    }

    fn first_hop(&self) -> Option<NodeAddress> {
        self.hops.first().copied()
    }
}

/// A Block holds the list of nodes that "keep track" of that block.
#[derive(Clone, Debug, Default)]
pub struct Block {
    owners: Vec<NodeAddress>,
}

impl Block {
    pub fn add_owner(&mut self, addr: NodeAddress) {
        if !self.owners.contains(&addr) {
            self.owners.push(addr);
        }
    }

    /// Return a Path to any owner that we already know about in `tracked_nodes`.
    /// We try owners in order and return the first Path we find in `tracked_nodes`.
    pub fn get_path_to_a_owner(
        &self,
        tracked_nodes: &HashMap<NodeAddress, Path>,
    ) -> Option<Path> {
        for &owner in &self.owners {
            if let Some(path) = tracked_nodes.get(&owner) {
                return Some(path.clone());
            }
        }
        None
    }

    /// Helper: pick the owner closest by XOR to a target address (no path).
    pub fn best_owner_by_xor(&self, dest: NodeAddress) -> Option<NodeAddress> {
        self.owners.iter().copied().min_by_key(|&a| a ^ dest)
    }
}

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
    pub fn new(_my_address: NodeAddress, canopy_start: StartOfAddressRange) -> Self {
        Self {
            _my_address,
            neighbors: HashSet::new(),
            root: array::from_fn(|_| Block::default()),
            levels: array::from_fn(|_| {
                array::from_fn(|_| (0u64, Block::default()))
            }),
            canopy: canopy_start,
            tracked_nodes: HashMap::new(),
        }
    }

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

    /// Check whether an address falls inside this node's canopy range.
    /// Adjust canopy size to your scheme; here we use 128 addresses as in your sketch.
    pub fn in_canopy(&self, addr: NodeAddress) -> bool {
        let start = self.canopy;
        let end = start.saturating_add(CROWN_SIZE);
        (start <= addr) && (addr < end)
    }

    /// Find the next Path toward `address` according to:
    /// 1. If we have a direct tracked path, return it.
    /// 2. If address is inside our canopy but not tracked, return None (node offline).
    /// 3. Search levels from fine to coarse; if a block has owners and we know a path to one, return it.
    /// 4. Fallback to root blocks and return a path if known.
    /// Returns Some(Path) when we can forward along a known intra-block chain,
    /// or None when the node is expected to be in our canopy but we have no record.
    pub fn find_next_path(&self, address: NodeAddress) -> Option<Path> {
        // 1) direct tracked path
        if let Some(path) = self.tracked_nodes.get(&address) {
            return Some(path.clone());
        }

        // 2) canopy check: if address is in canopy but not tracked, treat as offline
        if self.in_canopy(address) {
            return None;
        }

        // 3) search levels from finest (0) to coarsest (LEVELS-1)
        // width is the size of each block at the current level in address units.
        // Start with width = 128 (your base chunk size) and multiply per level.
        let mut width: u64 = 128;
        for level_idx in 0..LEVELS {
            width = width.saturating_mul(128); // block width at this level
            for (start, block) in &self.levels[level_idx] {
                let start = *start;
                if address >= start && address < start.saturating_add(width) {
                    // If we know a path to any owner of this block, return it
                    if let Some(p) = block.get_path_to_a_owner(&self.tracked_nodes) {
                        return Some(p);
                    } else {
                        // If we don't have a path to owners, but block has owners,
                        // we could return a direct owner (no path) or continue searching.
                        // Here we prefer to continue searching coarser levels.
                    }
                }
            }
        }

        // 4) fallback to root blocks
        // Choose root index from highest 8 bits
        let root_idx = (address >> (8 * 7)) as usize;
        if root_idx < ROOT_SIZE {
            let root_block = &self.root[root_idx];
            if let Some(p) = root_block.get_path_to_a_owner(&self.tracked_nodes) {
                return Some(p);
            }
            // If no path known, but owners exist, we could return a Path with the owner as direct hop:
            if let Some(owner) = root_block.best_owner_by_xor(address) {
                let mut p = Path::new();
                p.push(owner);
                return Some(p);
            }
        }

        // Nothing known
        None
    }
}

fn main() {
    // Add to Cargo.toml: array_init = "2.0"
    // Example usage
    let mut node = Node::new(0x0100_0000_0000_0001, 0x0100_0000);

    // Example: add a tracked node with a path
    let mut p = Path::new();
    p.push(0x0200_0000_0000_0002); // next hop
    node.add_tracked_node(0x0300_0000_0000_0003, p);

    // Query
    if let Some(path) = node.find_next_path(0x0300_0000_0000_0003) {
        let first_hop = path.first_hop();
        println!("Next hop toward target: {first_hop:?}", );
        if let Some(hop) = first_hop {
            if node.neighbors.contains(&hop) {
                 println!("which is a neighbor");
            } else {
                 println!("which is not a neighbor");
            }

        }
    } else {
        println!("No path known or node offline in canopy");
    }
}
