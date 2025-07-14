use std::collections::{BTreeSet, HashMap};

pub fn detect_cycles<
    Id: std::hash::Hash + std::cmp::Eq + std::cmp::Ord + Clone + std::fmt::Debug,
>(
    graph: &HashMap<Id, Id>,
) -> bool {
    detect_cycles_with_roots(graph).0
}
pub fn detect_cycles_with_roots<
    Id: std::hash::Hash + std::cmp::Eq + std::cmp::Ord + Clone + std::fmt::Debug,
>(
    graph: &HashMap<Id, Id>,
) -> (bool, BTreeSet<Id>) {
    let mut roots = BTreeSet::new();
    let mut visited = Vec::new();
    let mut unvisited: Vec<Id> = graph.keys().map(|x| x.clone()).collect();

    loop {
        let mut this_path = Vec::new();
        let Some(mut next_id) = unvisited.pop() else {
            return (false, roots);
        };
        'inner: loop {
            if this_path.contains(&next_id) {
                return (true, roots);
            };

            if visited.contains(&next_id) {
                // we have reached the tree so we can quit
                visited.push(next_id);
                break 'inner;
            };
            this_path.push(next_id.clone());
            let Some(parent) = graph.get(&next_id) else {
                //println!("45:found tree rooted as {next_id:?}");
                roots.insert(next_id.clone());
                visited.push(next_id);
                break 'inner;
            };
            next_id = parent.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn no_cycles() {
        let graph = HashMap::from([
            (10, 9),
            (9, 7),
            (8, 7),
            (7, 6),
            (6, 5),
            (5, 4),
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            //(4, 0),
        ]);

        assert_eq!(false, detect_cycles(&graph));
    }
    #[test]
    fn has_cycle() {
        let graph = HashMap::from([
            (10, 9),
            (9, 7),
            (8, 7),
            (7, 6),
            (6, 5),
            (5, 4),
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 0),
        ]);

        assert_eq!(true, detect_cycles(&graph));
    }
}
