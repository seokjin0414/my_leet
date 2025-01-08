use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    pub fn equal_digit_frequency(s: String) -> i32 {
        let s: Vec<usize> = s.chars()
            .map(|ch| ch.to_digit(10)
            .expect("chars must be digit between [0-9]") as usize)
            .collect();
            
        let n = s.len();
        let mut count = 0;
        let mut unique_substring: HashSet<u64> = HashSet::new();
        for i in 0..n {
            let mut freq = [0; 10];
            let (mut unique, mut mx, mut rolling_hash) = (0,0,0);

            for j in i..n {
                let d = s[j];
                if freq[d] == 0 {
                    unique+=1;
                }
                freq[d]+=1;
                mx = max(mx, freq[d]);
                rolling_hash = (13 * rolling_hash + d as u64 + 1) % 1_000_000_007;
                if mx*unique == (j - i + 1) {
                    unique_substring.insert(rolling_hash);
                }
            }
        }
        unique_substring.len() as i32
    }
}