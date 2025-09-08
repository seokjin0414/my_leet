use std::collections::BTreeMap;

impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut counts = BTreeMap::new();
        
        nums.iter().for_each(|n| *counts.entry(*n).or_insert(0) += 1);
        
        while let Some((k, v)) = counts.pop_last() {
            if v == 1 { return k; }
        }
        -1
    }
}