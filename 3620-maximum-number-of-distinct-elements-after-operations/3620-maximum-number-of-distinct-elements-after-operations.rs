impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums[0] -= k;
        for i in 1..nums.len() {
            nums[i] -= (nums[i] - nums[i - 1] - 1).clamp(-k, k);
        }
        nums.windows(2).filter(|win| win[1] != win[0]).count() as i32 + 1
    }
}