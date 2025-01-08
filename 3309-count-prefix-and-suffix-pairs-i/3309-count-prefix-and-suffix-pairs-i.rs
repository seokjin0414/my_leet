impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        words
            .iter()
            .enumerate()
            .flat_map(|(i, word)| {
                words[i + 1..].iter().filter(move |other_word| {
                    other_word.starts_with(&word[..]) && other_word.ends_with(&word[..])
                })
            })
            .count() as _
    }
}