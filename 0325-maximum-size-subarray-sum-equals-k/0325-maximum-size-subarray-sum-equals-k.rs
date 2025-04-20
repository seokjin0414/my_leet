use std::collections::HashMap;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let m = nums.len();                             // Length of nums
        let mut map : HashMap<i32,i32> = HashMap::new();// Hashmap that maps sum to leftmost appearance in sum_left
        let mut ret = 0;                                
        let mut total : i32 = 0;
        for (i,&num) in nums.iter().enumerate().rev() {
            total += num;
            if let Some(value) = map.get(&(total - k)) {
                ret = std::cmp::max(*value - i as i32, ret);
            }
            if total == k {
                ret = (m - i) as i32;
            }
            map.entry(total).or_insert(i as i32);
        }
        return ret;
    }
}