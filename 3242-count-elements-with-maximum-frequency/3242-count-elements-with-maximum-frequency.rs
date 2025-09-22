use std::collections::HashMap;
impl Solution {    
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut hash:HashMap<i32,i32> = HashMap::new();
        let max = nums.iter().fold((0,&mut hash),|(max,hash),&num| {
            *hash.entry(num).or_insert(0) += 1;
            (max.max(hash[&num]),hash)
        }).0;        
        hash.iter().fold(0,|result,(_,&freq)| if freq == max { result + freq} else {result})
    }
}