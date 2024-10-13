use std::collections::{BinaryHeap};
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut n = 0;

        for i in 0..nums.len() {
            heap.push((-nums[i][0], i, 0));
            n += nums[i].len();
        }

        let mut s = Vec::with_capacity(n);
        let mut counts = vec![0; nums.len()];
        let mut count = 0;
        let mut left = 0;
        let mut best = [-100000, 100000];

        while let Some((val, i, j)) = heap.pop() {
            let val = -val;
            s.push((val, i));

            if j+1 < nums[i].len() {
                heap.push((-nums[i][j+1], i, j+1));
            }

            counts[i] += 1;

            if counts[i] == 1 {
                count += 1
            }

            while count == nums.len() {
                let (v, k) = s[left];
                
                if val - v < best[1] - best[0] {
                    best = [v, val];
                }

                counts[k] -= 1;

                if counts[k] == 0 {
                    count -= 1
                }
                
                left += 1;
            }
        }
        best.to_vec()
    }
}