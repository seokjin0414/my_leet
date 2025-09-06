impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;

        for q in &queries {
            let (l, r) = (q[0] as i64, q[1] as i64);
            let (mut ops, mut start) = (0, 1);

            while start <= r {
                ops += r - l.max(start) + 1;
                start *= 4;
            }

            ans += (ops + 1) / 2;
        }

        ans
    }
}