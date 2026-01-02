impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 2..nums.len() {
            if nums[i] == nums[i - 1] || 
               nums[i] == nums[i - 2] {
                return nums[i];
            }
        }
        return nums[0]
    }
}