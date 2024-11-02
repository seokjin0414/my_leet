impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let chars: Vec<char> = sentence.chars().collect();
        let l = chars.len();
        
        if chars[0] != chars[l - 1] {
            return false
        }

        for i in 1..l-1 {
            if chars[i] == ' ' && chars[i-1] != chars[i+1] {
                return false
            }
        }
        
        true
    }
}