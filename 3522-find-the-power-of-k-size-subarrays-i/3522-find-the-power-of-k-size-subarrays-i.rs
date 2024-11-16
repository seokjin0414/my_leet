impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut result = vec![0; n-k+1];
        let mut left = 0;
        for right in k-1..n {
            for i in left..right {
                if nums[i] + 1 != nums[i + 1] {
                    result[left] = -1;
                    break;
                }
            }
            if result[left] != -1 {
                result[left] = nums[right];
            }
            left += 1
        }

        result
    }
}