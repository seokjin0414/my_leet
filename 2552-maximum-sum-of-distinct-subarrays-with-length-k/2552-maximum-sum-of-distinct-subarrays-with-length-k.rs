use std::collections::HashSet;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut elements = HashSet::new();
        let mut current_sum: i64 = 0;
        let mut max_sum: i64 = 0;
        let mut begin = 0;

        for end in 0..n {
            if !elements.contains(&nums[end]) {
                current_sum += nums[end] as i64;
                elements.insert(nums[end]);

                if end - begin + 1 == k {
                    max_sum = max_sum.max(current_sum);
                    current_sum -= nums[begin] as i64;
                    elements.remove(&nums[begin]);
                    begin += 1
                }
            } else {
                while nums[begin] != nums[end] {
                    current_sum -= nums[begin] as i64;
                    elements.remove(&nums[begin]);
                    begin += 1
                }
                begin += 1
            }
        }

        max_sum
    }
}