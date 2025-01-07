impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        
        for i in 0..words.len() {
            for j in i+1..words.len() {
                if words[j].len() < words[i].len() {
                    break;
                }
                
                if words[j].contains(&words[i]) {
                    res.push(words[i].clone());
                    break;
                }
            }
        }
        res
    }
}