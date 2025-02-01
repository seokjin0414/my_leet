impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|win| win[0] & 1 != win[1] & 1)
    }
}