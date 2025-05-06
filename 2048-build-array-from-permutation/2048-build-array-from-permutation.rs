impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;

        for i in 0..nums.len() {
            if nums[i] < 0 {
                continue;
            }

            if nums[i] == i as i32 {
                nums[i] -= n;
                continue;
            }

            let mut curr_i = i;
            let root = nums[i];
            while nums[nums[curr_i] as usize] >= 0 {
                let next_i = nums[curr_i] as usize;
                nums[curr_i] = nums[next_i] - n;
                curr_i = next_i;
            }

            nums[curr_i] = root - n;
        }

        for num in &mut nums {
            *num += n;
        }

        nums
    }
}