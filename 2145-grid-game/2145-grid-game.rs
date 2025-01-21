impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut row_1 = vec![0_i64; n+1];
        let mut row_2 = vec![0_i64; n+1];

        for idx in 1..=n {
            row_1[idx] = grid[0][idx - 1] as i64 + row_1[idx - 1];
            row_2[idx] = grid[1][idx - 1] as i64 + row_2[idx - 1];
        }

        (0..n).fold(i64::MAX, |robot_2, idx| {
            robot_2.min(row_2[idx].max(row_1[n] - row_1[idx + 1]))
        })
    }
}