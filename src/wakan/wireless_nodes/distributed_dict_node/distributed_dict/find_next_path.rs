use crate::wakan::wireless_nodes::distributed_dict_node::{
    DistributedDict, NodeAddress, Path, LEVELS, ROOT_SIZE,
};

impl DistributedDict {
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
