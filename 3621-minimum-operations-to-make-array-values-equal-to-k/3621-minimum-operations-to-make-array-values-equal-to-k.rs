impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BTreeSet;

        if nums.iter().any(|&x| x < k) {
            return -1;
        }

        let mut greater = BTreeSet::new();
        for &n in &nums {
            if n > k {
                greater.insert(n);
            }
        }

        greater.len() as i32
    }
}