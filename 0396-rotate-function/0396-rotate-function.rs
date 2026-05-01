impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut del_sum = nums
            .iter()
            .sum::<i32>() - nums[0];

        let mut res = nums
            .iter()
            .enumerate()
            .map(|(a, &b)| a as i32 * b)
            .sum::<i32>();

        let mut ans = res;

        for i in 0..n-1 {
            res = res - del_sum + (n - 1) as i32 * nums[i];
            ans = ans.max(res);
            del_sum = del_sum + nums[i] - nums[i + 1];
        }

        ans
    }
}