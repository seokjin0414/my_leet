impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {        
        let (mut prev,mut result,mut sum) = (nums[0],nums[0],nums[0]);
        for &num in nums.iter().skip(1) {
            if num <= prev { sum = 0; }
            sum += num;
            result = result.max(sum);
            prev = num;
        }
        result
    }
}