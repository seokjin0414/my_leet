impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut merge_string = String::with_capacity(word1.len() + word2.len());

        let mut word1 = word1.chars();
        let mut word2 = word2.chars();

        loop {
            match (word1.next(), word2.next()) {
                (Some(a), Some(b)) => {
                    merge_string.push(a);
                    merge_string.push(b);
                }
                (Some(a), None) => {
                    merge_string.push(a);
                   
                }
                (None, Some(b)) => {
                    merge_string.push(b);
                 
                }
                (None, None) => break,
            }
        }
        merge_string
    }
}