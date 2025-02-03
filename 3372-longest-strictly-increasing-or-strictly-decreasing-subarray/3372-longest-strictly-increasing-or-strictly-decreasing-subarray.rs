impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        nums.windows(2)
            .fold([1, 1, 1], |[i, d, m], win| match win[0].cmp(&win[1]) {
                std::cmp::Ordering::Less => [1, d + 1, m.max(d + 1)],
                std::cmp::Ordering::Equal => [1, 1, m],
                std::cmp::Ordering::Greater => [i + 1, 1, m.max(i + 1)],
            })[2]
    }
}