impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowels = ['a', 'e', 'i', 'o', 'u'].iter().copied().collect::<std::collections::HashSet<_>>();

        let mut prefix_sum = vec![0; words.len() + 1];
        for (i, word) in words.iter().enumerate() {
            let is_vowel_string = word.chars().next().map_or(false, |c| vowels.contains(&c)) &&
                                  word.chars().last().map_or(false, |c| vowels.contains(&c));
            prefix_sum[i + 1] = prefix_sum[i] + if is_vowel_string { 1 } else { 0 };
        }

        queries.into_iter().map(|query| {
            let left = query[0] as usize;
            let right = query[1] as usize;
            prefix_sum[right + 1] - prefix_sum[left]
        }).collect()
    }
}