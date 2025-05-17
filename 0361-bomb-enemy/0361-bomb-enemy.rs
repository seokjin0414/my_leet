impl Solution {
    pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut max_count: i32 = 0;
        let mut row_hits: i32 = 0;
        let mut col_hits: Vec<i32> = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if j == 0 || grid[i][j - 1] == 'W' {
                    row_hits = 0;
                    for k in j..n {
                        if grid[i][k] == 'W' {
                            break;
                        } else if grid[i][k] == 'E' {
                            row_hits += 1;
                        }
                    }
                }

                if i == 0 || grid[i - 1][j] == 'W' {
                    col_hits[j] = 0;
                    for k in i..m {
                        if grid[k][j] == 'W' {
                            break;
                        } else if grid[k][j] == 'E' {
                            col_hits[j] += 1;
                        }
                    }
                }

                if grid[i][j] == '0' {
                    max_count = std::cmp::max(max_count, row_hits + col_hits[j]);
                }
            }
        }

        max_count
    }
}