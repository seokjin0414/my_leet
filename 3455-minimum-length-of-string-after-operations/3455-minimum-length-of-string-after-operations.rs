impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut counts = [0; 26];

        for ch in s.chars() {
            let idx = ch as usize - 'a' as usize;

            counts[idx] += if counts[idx] < 2 { 1 } else { -1 };
        }

        counts.iter().sum()
    }
}