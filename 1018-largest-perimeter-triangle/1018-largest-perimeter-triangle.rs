impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        
        nums.sort_unstable();
        nums.reverse();

        nums.windows(3)
            .find(|&win| win[0] < win[1] + win[2])
            .map(|win| win.iter().sum())
            .unwrap_or(0)
    }
}