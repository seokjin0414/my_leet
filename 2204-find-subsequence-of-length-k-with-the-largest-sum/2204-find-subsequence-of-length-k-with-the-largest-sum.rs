impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut idx: Vec<usize> = (0..nums.len()).collect();

        idx.sort_unstable_by_key(|&i| -nums[i]);

        let mut sel = idx[..k].to_vec();
        sel.sort_unstable();

        sel.into_iter().map(|i| nums[i]).collect()
    }
}