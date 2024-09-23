use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let max = s.len() as i32 + 1;
        let mut dp = vec![max; s.len() + 1];
        dp[0] = 0;

        let dictionary_set: HashSet<_> = dictionary.into_iter().collect();

        for i in 1..=s.len() {
            dp[i] = dp[i - 1] + 1;
            for l in 1..=i {
                if dictionary_set.contains(&s[i-l..i].to_string()) {
                    dp[i] = std::cmp::min(dp[i], dp[i-l]);
                }
            }
        }
        dp[s.len()] as i32
    }
}
