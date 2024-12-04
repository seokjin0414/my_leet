impl Solution {
    pub fn can_make_subsequence(a: String, b: String) -> bool {
        let mut j = 0;
        let mut i = 0;

        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        while i < a_chars.len() && j < b_chars.len() {
            if a_chars[i] == b_chars[j] || 
                (a_chars[i] as u8 - 'a' as u8 == ((b_chars[j] as u8 - 'a' as u8 + 26) - 1) % 26) {
                j += 1;
            }
            i += 1;
        }

        j == b_chars.len()
    }
}