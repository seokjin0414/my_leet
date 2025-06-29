const MOD: u64 = 1000000007;

const POW_2: [u32; 100001] = {
    let mut pow_2 = [1; 100001];
    let mut i = 0;
    while i < 100000 {
        pow_2[i + 1] = ((pow_2[i] as u64 * 2) % MOD) as u32;
        i += 1;
    }
    pow_2
};

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let min = nums.iter().min().unwrap();
        let mut nums: Vec<i32> = nums
            .iter()
            .filter(|&num| *num <= target - min)
            .cloned()
            .collect();
        nums.sort();
        if nums.is_empty() {
            0
        } else {
            let mut j = nums.len() - 1;
            let mut ret = POW_2[j] as u64;
            'outer: for (i, num) in nums.iter().enumerate().skip(1) {
                if i > j {
                    break 'outer;
                }
                while num + nums[j] > target {
                    j -= 1;
                    if i > j {
                        break 'outer;
                    }
                }
                ret += POW_2[j - i] as u64;
            }
            (ret % MOD) as i32
        }
    }
}