impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as usize;
        let cume_sums: Vec<_> = nums.iter().scan(0, |tot, &v| {
            *tot += v as usize;
            Some(*tot)
        }).collect();
        
        let full_sum = *cume_sums.last().unwrap();

        if full_sum % p == 0 {
            return 0;
        }
        
        if nums.iter().any(|&v| (full_sum - v as usize) % p == 0) {
            return 1;
        }
        
        for win in 2..nums.len() {
            if (0..=nums.len() - win).any(|a| {
                let start = if a == 0 { 0 } else { cume_sums[a - 1] };
                let end = cume_sums[a + win - 1];
                (full_sum - end + start) % p == 0
            }) {
                return win as i32;
            }
        }
        
        -1
    }
}