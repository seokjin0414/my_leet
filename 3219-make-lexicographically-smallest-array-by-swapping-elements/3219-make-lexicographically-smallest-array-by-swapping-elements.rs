impl Solution {
    pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut tups: Vec<(usize, i32)> = nums.iter().copied().enumerate().collect();
        tups.sort_unstable_by_key(|k| k.1);
    
        for chunk in tups.chunk_by(|a, b| b.1 - a.1 <= limit) {
            let mut indices = chunk.to_vec();
            indices.sort_unstable();

            for i in 0..indices.len() {
                let idx = indices[i].0;
                nums[idx] = chunk[i].1;
            }
        }

        nums
    }
}