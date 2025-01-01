impl Solution {
    pub fn max_score(s: String) -> i32 {
        let ones = s.chars().filter(|&c| c == '1').count() as i32;
        let mut res = 0;
        let mut ones_remaining = ones;

        let zeroes = s.chars().take(s.len() - 1).fold(0, |mut acc, c| {
            if c == '0' {
                acc += 1;
            } else {
                ones_remaining -= 1;
            }
            res = res.max(acc + ones_remaining);

            acc
        });

        res
    }
}