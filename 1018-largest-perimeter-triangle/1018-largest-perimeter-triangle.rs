impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        for idx in 0..nums.len() - 2 {
            if nums[idx] + nums[idx + 1] > nums[idx + 2] {
                result = nums[idx] + nums[idx + 1] + nums[idx + 2];
            }
        }
        result
    }
}