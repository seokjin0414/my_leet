impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        nums.into_iter()
            .try_fold((0, i32::MAX), |(max, min), n| {
                Some((max.max(n), min.min(n)))
            })
            .map_or(0, |(max, min)| i64::from(max - min) * i64::from(k))
    }
}