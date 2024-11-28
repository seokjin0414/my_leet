use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = VecDeque::<(usize, usize, i32)>::new();
        q.push_back((0, 0, grid[0][0]));
        grid[0][0] = -1;
        while let Some((i, j, step)) = q.pop_front() {
            for w in [0, 1, 0, usize::MAX, 0].windows(2) {
                let i2 = i.wrapping_add(w[0]);
                let j2 = j.wrapping_add(w[1]);
                if i2 == grid.len() - 1 && j2 == grid[0].len() - 1 {
                    return step + grid[i2][j2];
                }
                if i2 >= grid.len() || j2 >= grid[0].len() {
                    continue;
                }
                if grid[i2][j2] == 0 {
                    grid[i2][j2] = -1;
                    q.push_front((i2, j2, step));
                }
                else if grid[i2][j2] == 1 {
                    grid[i2][j2] = -1;
                    q.push_back((i2, j2, step + 1));
                }
            }
        }
        unreachable!()
    }
}