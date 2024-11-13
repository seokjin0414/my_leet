impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let len = nums.len();
        nums.sort_unstable();
        (0..len - 1)
            .map(|i| {
                let hi = nums[i + 1..].partition_point(|&j| nums[i] + j <= upper);
                if hi == 0 { return 0; }
                let lo = nums[i + 1..].partition_point(|&j| nums[i] + j < lower);
                hi.saturating_sub(lo) as i64
            })
            .sum()
    }
}