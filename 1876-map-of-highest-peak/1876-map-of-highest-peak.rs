impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (is_water.len(), is_water[0].len());

        let mut visited = vec![vec![false; n]; m];
        let mut queue = VecDeque::new();
        let mut new_map = is_water;
        for (i, (map_row, visited_row)) in new_map.iter_mut().zip(&mut visited).enumerate() {
            for (j, (map_cell, visited_cell)) in map_row.iter_mut().zip(visited_row).enumerate() {
                if replace(map_cell, 0) == 1 {
                    queue.push_back((i, j));
                    *visited_cell = true;
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            if i > 0 && !replace(&mut visited[i - 1][j], true) {
                new_map[i - 1][j] = new_map[i][j] + 1;
                queue.push_back((i - 1, j));
            }

            if i < m - 1 && !replace(&mut visited[i + 1][j], true) {
                new_map[i + 1][j] = new_map[i][j] + 1;
                queue.push_back((i + 1, j));
            }

            if j > 0 && !replace(&mut visited[i][j - 1], true) {
                new_map[i][j - 1] = new_map[i][j] + 1;
                queue.push_back((i, j - 1));
            }

            if j < n - 1 && !replace(&mut visited[i][j + 1], true) {
                new_map[i][j + 1] = new_map[i][j] + 1;
                queue.push_back((i, j + 1));
            }
        }

        new_map
    }
}

use std::{collections::VecDeque, mem::replace};