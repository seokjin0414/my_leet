impl Solution {
    pub fn rotate_the_box(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows = grid.len();
        let cols = grid[0].len();
        
        let mut res = vec![vec!['.'; rows]; cols];
        
        for r in 0..rows {
            let mut i = cols - 1;
            for c in (0..cols).rev() {
                if grid[r][c] == '#' {
                    res[i][rows - r - 1] = '#';
                    i = i.saturating_sub(1);
                } else if grid[r][c] == '*' {
                    res[c][rows - r - 1] = '*';
                    i = c.saturating_sub(1);
                }
            }
        }
        res
    }
}