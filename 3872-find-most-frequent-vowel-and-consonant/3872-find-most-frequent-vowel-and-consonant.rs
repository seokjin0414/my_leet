impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut vowels = [0;26];
        let mut consonants = [0;26];
        fn is_vowel(b: u8) -> bool {
            b"aeiou".contains(&b)
        }
        for b in s.bytes() {
            if is_vowel(b) {
                vowels[(b-b'a') as usize] +=1
            }
            else {
                consonants[(b-b'a') as usize] +=1
            }
        }
        vowels.into_iter().max().unwrap_or(0) + consonants.into_iter().max().unwrap_or(0)
    }
}