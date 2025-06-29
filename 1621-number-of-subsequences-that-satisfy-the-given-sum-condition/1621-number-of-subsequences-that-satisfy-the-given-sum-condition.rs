impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = nums.len();
        nums.sort();

        let mut power = vec![1; n];
        for i in 1..n {
            power[i] = (power[i - 1] * 2) % MOD;
        }

        let mut left = 0;
        let mut right = n - 1;
        let mut result = 0;

        while left <= right {
            if nums[left] + nums[right] <= target {
                result = (result + power[right - left]) % MOD;
                left += 1;
            } else {
                if right == 0 { break; }
                right -= 1;
            }
        }

        result
    }
}