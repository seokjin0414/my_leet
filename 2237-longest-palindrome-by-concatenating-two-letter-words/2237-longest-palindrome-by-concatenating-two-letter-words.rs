impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut open = [0; 26 * 26];
        let mut total = 0;
        for word in words {
            let a = word.as_bytes()[0] as usize - 'a' as usize;
            let b = word.as_bytes()[1] as usize - 'a' as usize;
            let hash = a * 26 + b;
            let inv_hash = b * 26 + a;
            
            if open[inv_hash] > 0 {
                open[inv_hash] -= 1;
                total += 4;
            } else {
                open[hash] += 1;
            }
        }
        for i in 0..26 {
            if open[i * 26 + i] > 0 {
                total += 2;
                break;
            }
        }

        total
    }
}