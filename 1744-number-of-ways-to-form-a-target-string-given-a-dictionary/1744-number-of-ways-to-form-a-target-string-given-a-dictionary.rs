use std::collections::VecDeque;

pub fn preproc(words: Vec<String>) -> Vec<[usize; 32]> {
    let n = words[0].len();

    let mut freq_map = vec![[0; 32]; n]; // 28 chars in alphabet

    for word in words {
        for (i, c) in word.chars().enumerate() {
            let c = c as usize - 'a' as usize;
            freq_map[i][c] += 1;
        }
    }

    freq_map
}

impl Solution {
    const MOD: usize = 1_000_000_007;

    pub fn _impl(words: Vec<String>, target: String) -> Option<usize> {
        if words.is_empty() { return None; }
        
        let mut target = target.chars()
            .map(|c| c as usize - 'a' as usize)
            .collect::<Vec<_>>(); // convert string to vec of chars
        let mut freq_map = preproc(words); // frequency map for each index

        let (m, n) = (target.len(), freq_map.len());
        if m > n { return None; }

        target.reverse();
        freq_map.reverse(); // WHY DO YOU HATE YOURSELF

        let dp_len = n - m + 1;
        let mut dp = VecDeque::from(vec![1usize; dp_len]); // magic

        for (i, c) in target.into_iter().enumerate() { // what??
            let mut count_prev = 0usize;

            for j in (0..(n-m+1)) { // ??
                let mut head = dp.pop_front().unwrap();

                head *= freq_map[i + j][c]; // WAT?????
                head += count_prev;
                head %= Self::MOD;

                dp.push_back(head);

                count_prev = head;
            }
        }

        dp.pop_back()
    }

    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        Self::_impl(words, target)
            .unwrap_or(0) as i32
    }
}