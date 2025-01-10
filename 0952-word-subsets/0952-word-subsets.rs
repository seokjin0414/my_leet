impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let id = |i: u8| (i - b'a') as usize;
        let count = |s: &str| -> [i32; 26] {
            let mut counter = [0i32; 26];
            
            for ch in s.as_bytes() {
                counter[id(*ch)] += 1;
            }
            counter
        };
        let mut letters = [0i32; 26];
        
        for word in words2 {
            let mut temp = count(&word);
            
            for ch in word.as_bytes() {
                letters[id(*ch)] = std::cmp::max(letters[id(*ch)], temp[id(*ch)]);
            }
        }
        
        let mut res = Vec::new();
        'outer: for word in words1 {
            let mut letters1 = count(&word);
            
            for i in 0..26 {
                if letters[i] > letters1[i] {
                    continue 'outer;
                }
            }
            res.push(word);
        }
        res
    }
}