impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (num_rows, num_cols) = (grid.len(), grid[0].len());

        let col_counts = (0..num_cols)
          .map(|i| (0..num_rows)
            .map(|j| grid[j][i])
            .sum::<i32>()
          )
          .collect::<Vec<_>>();
        
        let row_counts = (0..num_rows)
          .map(|i| (0..num_cols)
            .map(|j| grid[i][j])
            .sum::<i32>()
          )
          .collect::<Vec<_>>();

        (0..num_rows)
          .flat_map(|i| (0..num_cols).map(move |j| (i, j)))
          .filter(|&(i, j)| row_counts[i] > 1 || col_counts[j] > 1)
          .map(|(i, j)| grid[i][j])
          .sum()
    }
}