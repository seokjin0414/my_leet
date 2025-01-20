use std::collections::VecDeque;

impl Solution {

    pub fn get_food(grid: Vec<Vec<char>>) -> i32 {
        const VISITED: char = 'V';
        let mut grid = grid;
        let (num_rows, num_cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut start: (usize, usize) = (0, 0);

        for (r, row) in grid.iter().enumerate() {
            for (c, ch) in row.iter().enumerate() {
                if *ch == '*' {
                    start = (r, c);
                }
            }
        }

        let mut q = VecDeque::new();
        q.push_back(((start.0, start.1), 0i32));
        grid[start.0][start.1] = VISITED;

        while let Some(pos) = q.pop_front() {
            let (row, col, dist) = (pos.0.0, pos.0.1, pos.1);
            for delta in [0, 1, 0, -1, 0].windows(2) {
                let new_row = delta[0] + row as i32;
                let new_col = delta[1] + col as i32;

                if (0..num_rows).contains(&new_row) && (0..num_cols).contains(&new_col) {
                    let (new_row, new_col) = (new_row as usize, new_col as usize);
                    if grid[new_row][new_col] == '#' {
                        return dist + 1;
                    }
            
                    if grid[new_row][new_col] == 'O' {
                        q.push_back(((new_row, new_col), dist + 1));
                        grid[new_row][new_col] = VISITED;
                    }
                }
            }
        }

        -1
    }
}