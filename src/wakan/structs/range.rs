use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
    pub number_of_nodes: usize,
    pub seq: u64,
}

fn compute_error<T>(a: &Range<T>, b: &Range<T>) -> u64 {
    let (min, max) = if a.seq < b.seq { (a, b) } else { (b, a) };
    let dist = max.seq - min.seq;
    dist * max.number_of_nodes as u64
}

fn find_best_pair<T: Copy>(
    ranges: &[Range<T>],
) -> Option<((usize, usize), u64)> {
    ranges
        .iter()
        .enumerate()
        .array_combinations::<2>()
        .map(|pair| {
            let ((i, a), (j, b)) = (pair[0], pair[1]);
            let error = compute_error(a, b);
            ((i, j), error)
        })
        .min_by_key(|(_, error)| *error)
}

fn find_best_across<T: Copy>(
    a_ranges: &[Range<T>],
    b_ranges: &[Range<T>],
) -> Option<((usize, usize), u64)> {
    let mut min_error = u64::MAX;
    let mut best_pair = None;

    for (i, a) in a_ranges.iter().enumerate() {
        for (j, b) in b_ranges.iter().enumerate() {
            let error = compute_error(a, b);
            if error < min_error {
                min_error = error;
                best_pair = Some(((i, j), error));
            }
        }
    }

    best_pair
}

pub fn find_least_error_among_all<T: Copy>(
    a_ranges: &[Range<T>],
    b_ranges: &[Range<T>],
) -> Option<Case> {
    let best_a = find_best_pair(a_ranges).map(|((i, j), error)| Case::A((i, j), error));
    let best_b = find_best_pair(b_ranges).map(|((i, j), error)| Case::B((i, j), error));
    let best_cross = find_best_across(a_ranges, b_ranges)
        .map(|((i, j), error)| Case::C { a: i, b: j, error });

    [best_a, best_b, best_cross]
        .into_iter()
        .flatten()
        .min_by_key(|case| match case {
            Case::A(_, e) => *e,
            Case::B(_, e) => *e,
            Case::C { error, .. } => *error,
        })
}

pub enum Case {
    A((usize, usize), u64),
    B((usize, usize), u64),
    C { a: usize, b: usize, error: u64 },
}
