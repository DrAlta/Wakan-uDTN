use std::collections::BTreeMap;

use crate::wakan::wireless_nodes::distributed_dict_node::{
    Block, NodeAddress, Path, StartOfAddressRange, LEVELS, LEVEL_BUCKETS, ROOT_SIZE,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DistributedDict {
    // Dict stuff
    pub(super) root: [Block; ROOT_SIZE],
    pub(super) levels: [[(StartOfAddressRange, Block); LEVEL_BUCKETS]; LEVELS],
    pub(super) canopy: StartOfAddressRange,
    pub(super) tracked_nodes: BTreeMap<NodeAddress, Path>,
}

#[cfg(test)]
mod test {
    use std::array;
    use qol::assert_specimen;

    
    use super::*;
    #[test]
    fn find_test(){
        fn find(overdrive: &BTreeMap<StartOfAddressRange, (u8, Path)>, target: u64) -> Option<&Path> {
            // 1. Get a range of entries from the start up to the target (inclusive)
            // 2. Use .next_back() to get the entry with the largest key in that range
            overdrive
                .range(..=target)
                .next_back()
                .and_then(|(&start_address, (length, path))| {
                    // 3. Check if the target falls within [start_address, start_address + length)
                    // Note: We use u64 for the addition to prevent overflow if u8 is large
                    if target < start_address + (*length as u64) {
                        Some(path)
                    } else {
                        None
                    }
                })
        }


        let overdrive = BTreeMap::from([
            (5_u64, (20_u8, Path::from_raw(vec![100]))),
            (10, (5, Path::from_raw(vec![200]))),
        ]);
        assert_specimen!(
            find(&overdrive, 17),
            Some(
                &Path::from_raw(
                    vec![100]
                )
            )
        );
    }

    #[test]
    fn flags_have_capacity(){
        assert!((LEVEL_BUCKETS - 1) as u128 <= u128::MAX);
    }
    #[test]
    fn cano() {
        let path = Path::from_raw(vec![1,2,3,4,5]);
        let root= array::from_fn(|_| Block::default());
        let levels = array::from_fn(|_| array::from_fn(|_| (0u64, Block::default())));
        let distributed_dict = DistributedDict{ 
            root,
            levels,
            canopy: 0,
            tracked_nodes: BTreeMap::from([
                (42, path.clone())
            ])
        };
        let x = distributed_dict.find_next_path(42);
        assert_specimen!(
            x,
            Some(&path)
        )
    }
    #[test]
    fn level_0() {
        let path = Path::from_raw(vec![1,2,3,4,5]);
        let root= array::from_fn(|_| Block::default());
        let mut levels = array::from_fn(|_| array::from_fn(|_| (0u64, Block::default())));
        levels[0][0].0 = 190;
        levels[0][0].1.add_owner(42);
        let distributed_dict = DistributedDict{ 
            root,
            levels,
            canopy: 64000,
            tracked_nodes: BTreeMap::from([
                (42, path.clone())
            ])
        };
        let x = distributed_dict.find_next_path(200);
        assert_specimen!(
            x,
            Some(&path)
        )
    }
}