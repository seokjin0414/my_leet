use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut hash = HashMap::<Vec<i32>, i32>::new();
        let mut ans = 1;
        for mut v in matrix.into_iter() {
            if v[0] == 1 {
                for b in v.iter_mut() {
                    *b = 1 - *b;
                }
            }
            let entry = hash.entry(v)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            ans = ans.max(*entry);
        }
        ans
    }
}