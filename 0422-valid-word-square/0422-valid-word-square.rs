impl Solution {
    pub fn valid_word_square(words: Vec<String>) -> bool {
        let n = words.len();
        for i in 0..n {
            let m = words[i].len();
            if m > n {
                return false;
            }
            for j in 0..m {
                if words[j].len() <= i || words[i].as_bytes()[j] != words[j].as_bytes()[i] {
                    return false;
                }
            }
        }
        true
    }
}