use std::collections::{BTreeSet, HashMap};
use std::convert::{TryFrom, TryInto};

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        // Use better format of input data.
        let stones: Vec<(i32, i32)> = stones
            .into_iter()
            .map(|coord| {
                assert_eq!(coord.len(), 2);
                (coord[0], coord[1])
            })
            .collect();

        let mut row_to_first_idx = HashMap::with_capacity(stones.len());
        let mut col_to_first_idx = HashMap::with_capacity(stones.len());
        for (i, &(row, col)) in stones.iter().enumerate() {
            let i = u32::try_from(i).unwrap();
            row_to_first_idx.entry(row).or_insert(i);
            col_to_first_idx.entry(col).or_insert(i);
        }

        let n: u32 = stones.len().try_into().unwrap();
        let mut djs = DisjointSet::new(n);
        for (i, &(row, col)) in stones.iter().enumerate() {
            let i = u32::try_from(i).unwrap();
            let first_row_idx = *row_to_first_idx.get(&row).unwrap();
            let first_col_idx = *col_to_first_idx.get(&col).unwrap();
            djs.union(first_row_idx, i);
            djs.union(first_col_idx, i);
        }
        // Get each unique group.
        let unique_roots: BTreeSet<u32> = (0..n).map(|i| djs.get_root(i)).collect();
        // We keep 1 stone for each connected group.
        let remaining_stones = unique_roots.len();
        let existed_stones = stones.len();
        (existed_stones - remaining_stones).try_into().unwrap()
    }
}

struct DisjointSet {
    // Use one buffer for both roots and ranks
    // to minimize allocations.
    // See `fn roots_and_ranks`.
    data: Box<[u32]>,
}

#[inline]
#[track_caller]
fn u2s(idx: u32) -> usize {
    idx.try_into().unwrap()
}

impl DisjointSet {
    fn new(size: u32) -> Self {
        let double_size = usize::try_from(size).unwrap() * 2;
        let mut res = Self {
            data: vec![0; double_size].into(),
        };
        let (roots, ranks) = res.roots_and_ranks();
        for (i, v) in roots.iter_mut().enumerate() {
            *v = u32::try_from(i).unwrap();
        }
        ranks.fill(1);
        res
    }

    fn roots_and_ranks(&mut self) -> (&mut [u32], &mut [u32]) {
        let half_len = self.data.len() / 2;
        self.data.split_at_mut(half_len)
    }

    fn get_root(&mut self, position: u32) -> u32 {
        let (roots, _) = self.roots_and_ranks();
        let mut current = position;
        while roots[u2s(current)] != current {
            current = roots[u2s(current)];
        }
        let root = current;
        if roots[u2s(position)] == root {
            return root;
        }

        current = position;
        while current != root {
            let nxt = roots[u2s(current)];
            roots[u2s(current)] = root;
            current = nxt;
        }
        root
    }

    fn union(&mut self, f: u32, s: u32) {
        use std::cmp::Ordering;

        if f == s {
            return;
        }

        let root_f = self.get_root(f);
        let root_s = self.get_root(s);
        if root_f == root_s {
            return;
        }

        let (roots, ranks) = self.roots_and_ranks();
        let rank_f = ranks[u2s(root_f)];
        let rank_s = ranks[u2s(root_s)];
        match rank_f.cmp(&rank_s) {
            Ordering::Less => roots[u2s(root_f)] = root_s,
            Ordering::Greater => roots[u2s(root_s)] = root_f,
            Ordering::Equal => {
                let new_rank = rank_f + 1;
                roots[u2s(root_s)] = root_f;
                ranks[u2s(root_f)] = new_rank;
            }
        }
    }
}