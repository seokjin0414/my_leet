impl Solution {
    pub fn b(i: usize, mut v: i32, m: i32, nums: &Vec<i32>) -> i32 {
        if i == nums.len() {
            if v == m { return 1; }
            return 0;
        }
        Self::b(i+1, v | nums[i], m, nums) + Self::b(i+1, v, m, nums)
    }

    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let m: i32 = nums.iter().fold(0, |acc, v| acc | v);
        Self::b(0, 0, m, &nums)
    }
}