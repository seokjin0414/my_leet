impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let n = n as i64;
        let l = limit as i64;
        (0..=n.min(l)).map(|i| 0.max(1 + l.min(n-i) - 0.max(n-l-i))).sum()
    }
}