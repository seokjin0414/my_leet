use std::collections::HashSet;
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();
        for pattern in &words {
            for word in &words {
                if word == pattern { continue }
                if word.find(pattern).is_some() {
                    result.insert(pattern.into());
                }
            }
        }
        result.into_iter().collect()
    }
}