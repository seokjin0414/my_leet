use std::collections::VecDeque;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut q_incr = VecDeque::<usize>::new(); // Produces min value index.
        let mut q_decr = VecDeque::<usize>::new(); // Produces max value index.
        let mut start  = 0;
        let mut count  = 0;

        for end in 0..nums.len() {
            while q_decr.back().map_or(false, |&i| nums[i] <= nums[end]) {
                q_decr.pop_back();
            }
            q_decr.push_back(end);

            while q_incr.back().map_or(false, |&i| nums[i] >= nums[end]) {
                q_incr.pop_back();
            }
            q_incr.push_back(end);

            while let (Some(&min), Some(&max)) = (q_incr.front(), q_decr.front()) {
                if nums[max] - nums[min] > 2 {
                    let q = if max < min { &mut q_decr } else { &mut q_incr };
                    start = q.pop_front().unwrap() + 1;
                } else {
                    break;
                }
            }
            count += end + 1 - start;
        }

        count as i64
    }
}