
impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut vowel_frequencies = [0; 26];
        let mut consonant_frequencies = [0; 26];

        for char in s.bytes() {
            if char == b'a' || char == b'e' || char == b'i' || char == b'o' || char == b'u' {
                vowel_frequencies[(char - b'a') as usize] += 1;
            } else {
                consonant_frequencies[(char - b'a') as usize] += 1;
            }
        }

        vowel_frequencies.into_iter().max().unwrap_or(0)
            + consonant_frequencies.into_iter().max().unwrap_or(0)
    }
}