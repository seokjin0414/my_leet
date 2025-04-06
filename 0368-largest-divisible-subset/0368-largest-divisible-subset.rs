impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut group_size = vec![1; n];
        let mut prev_element = vec![-1; n];
        let mut max_index = 0;

        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && group_size[i] < group_size[j] + 1 {
                    group_size[i] = group_size[j] + 1;
                    prev_element[i] = j as i32;
                }
            }
            if group_size[i] > group_size[max_index] {
                max_index = i;
            }
        }

        let mut result = Vec::new();
        let mut i = max_index as i32;
        while i != -1 {
            result.insert(0, nums[i as usize]);
            i = prev_element[i as usize];
        }
        result
    }
}