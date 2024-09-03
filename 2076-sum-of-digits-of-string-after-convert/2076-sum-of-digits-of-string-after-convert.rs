impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
            let mut n = 0;
            
            for c in s.chars() {
                n += (c as i32 - 96) / 10 + (c as i32 - 96) % 10;
            }
            
            for i in 1..k {
                let mut tmp = 0;
                
                while n != 0 {
                    tmp += n % 10;
                    n /= 10;
                }
                n = tmp;
            }
            n
    }
}