use std::collections::VecDeque;

const adj: [usize; 8] = [0, 1, 0, usize::MAX, 1, 0, usize::MAX, 0];

impl Solution {
    pub fn min_cost(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::<(usize, usize, i32, bool)>::new();
        q.push_back((0, 0, 0, true));
        grid[0][0] *= -1;
        while let Some((i, j, val, is_arrow)) = q.pop_front() {
            if i == grid.len() - 1
            && j == grid[0].len() - 1 {
                return val;
            }
            if is_arrow {
                q.push_back((i, j, val, false));
            }
            let arrow = (-grid[i][j] - 1) as usize;
            for d in 0..4 {
                if (is_arrow && d == arrow)
                || (!is_arrow && d != arrow) {
                    let i2 = i.wrapping_add(adj[d * 2]);
                    let j2 = j.wrapping_add(adj[d * 2 + 1]);
                    if i2 < grid.len()
                    && j2 < grid[0].len()
                    && grid[i2][j2] > 0 {
                        grid[i2][j2] *= -1;
                        if is_arrow {
                            q.push_front((i2, j2, val, true));
                        }
                        else {
                            q.push_back((i2, j2, val + 1, true));
                        }
                    }
                }
            }
        }
        unreachable!()
    }
}