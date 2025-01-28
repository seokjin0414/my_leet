use std::cmp;

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_fish = 0;
        let m = grid.len();
        let n = grid[0].len();
        
        fn dfs(grid: &mut Vec<Vec<i32>>, i: isize, j: isize) -> i32 {
            let fish_count = grid[i as usize][j as usize];
            grid[i as usize][j as usize] = 0;
            let directions: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
            let mut total_fish = fish_count;
            for (dx, dy) in directions {
                let x = i + dx;
                let y = j + dy;
                if x >= 0 && x < grid.len() as isize && y >= 0 && y < grid[0].len() as isize && grid[x as usize][y as usize] > 0 {
                    total_fish += dfs(grid, x, y);
                }
            }
            total_fish
        }
        
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    max_fish = cmp::max(max_fish, dfs(&mut grid, i as isize, j as isize));
                }
            }
        }
        max_fish
    }
}