impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut new_x = x as i64;
        let mut to_merge = (n - 1) as i64;
        let mut merged = 0;
        let mut dist = 0;
       
        while to_merge > 0 {
            if new_x & 1 == 0 {
                merged |= (to_merge & 1) << dist;
                to_merge >>= 1;
            }
            dist += 1;
            new_x >>= 1;
        }
        merged | x as i64
    }
}