impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let n = n as usize;
        let i = first_player as usize - 1;
        let j = second_player as usize - 1;;
        let mut dp = vec![vec![vec![vec![-1; 2]; j + 1]; i + 1]; n + 1];
        Self::memo(n, i, j, &mut dp);
        dp[n][i].pop().unwrap()
    }

    fn memo(n: usize, i: usize, j: usize, dp: &mut Vec<Vec<Vec<Vec<i32>>>>) {
        if dp[n][i][j][0] != -1 {
            return;
        }
        if i + j + 1 == n {
            dp[n][i][j][0] = 1;
            dp[n][i][j][1] = 1;
            return;
        }
        let n2 = (n + 1) / 2;
        let i_seq = i.min(n - i - 1);
        let j_seq = j.min(n - j - 1);
        let mut l = 0;
        let mut m = 0;
        let mut lr = 0;
        let mut lm = 0;
        let mut mr = 0;
        for a in 0..n2 {
            if a == i_seq || a == j_seq {
                continue;
            }
            let b = n - a - 1;
            match (Self::get_region(a, i, j), Self::get_region(b, i, j)) {
                (0, 0) => l += 1,
                (0, 1) => lm += 1,
                (0, 2) => lr += 1,
                (1, 1) => m += 1,
                (1, 2) => mr += 1,
                (2, 2) => (),
                _ => unreachable!(),
            }
        }
        dp[n][i][j][0] = i32::MAX;
        dp[n][i][j][1] = i32::MIN;
        for lr_i in 0..=lr {
            for lm_i in 0..=lm {
                for mr_i in 0..=mr {
                    let i2 = l + lr_i + lm_i;
                    let j2 = l + m + lr_i + lm + mr_i + 1;
                    Self::memo(n2, i2, j2, dp);
                    dp[n][i][j][0] = dp[n][i][j][0].min(dp[n2][i2][j2][0] + 1);
                    dp[n][i][j][1] = dp[n][i][j][1].max(dp[n2][i2][j2][1] + 1);
                }
            }
        }
    }

    fn get_region(a: usize, i: usize, j: usize) -> i32 {
        if a < i { 0 } else if a < j { 1 } else { 2 }
    }
}