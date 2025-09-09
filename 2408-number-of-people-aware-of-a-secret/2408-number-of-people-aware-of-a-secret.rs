use std::collections::*;
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let MOD = 1e9 as i64 + 7;

        let mut counts = vec![0i64; n as usize + 1];
        let mut cur = 0;
        counts[1] = 1;
        for d in 2..=n {
            let forget_day = d - forget;
            if forget_day >= 1 {
                cur = ((cur - counts[forget_day as usize]) % MOD + MOD) % MOD;
                counts[forget_day as usize] = 0;
            }

            let delay_day = d - delay;
            if delay_day >= 1 {
                cur += counts[delay_day as usize];
                cur %= MOD;
                counts[d as usize] = cur;
            }
        }
        let mut ans = 0;
        for d in counts {
            ans += d;
            ans %= MOD;
        }
        ans as i32
    }
}