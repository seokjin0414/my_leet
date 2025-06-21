use std::collections::HashMap;

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        fn get_freqs(word: String) -> HashMap<char,i32> {
            let mut result:HashMap<char,i32> = HashMap::new();
            for ch in word.chars() {
                *result.entry(ch).or_insert(0) += 1;
            }
            result
        }
        let freqs = get_freqs(word);
        let mut freqs = freqs.iter().map(|(key,value)| *value).collect::<Vec<i32>>();
        freqs.sort();
        let mut result = i32::MAX;
        let mut accum = 0;
        for idx in 0..freqs.len() {
            let mut res = 0;
            for idx2 in idx+1..freqs.len() {
                res += if freqs[idx2] > freqs[idx] + k {freqs[idx2]-freqs[idx]-k} else {0};                
            }
            result = result.min(res+accum);
            accum += freqs[idx];
        }
        result
    }
}