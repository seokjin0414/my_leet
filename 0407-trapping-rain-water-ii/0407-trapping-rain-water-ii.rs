use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        if height_map.is_empty() || height_map[0].is_empty() {
            return 0;
        }

        let mut heap = BinaryHeap::new();
        let (rows, cols) = (height_map.len(), height_map[0].len());
        let mut visited = vec![vec![false; cols]; rows];
        let mut water_trapped = 0;

        for i in 0..rows {
            heap.push((Reverse(height_map[i][0]), (i, 0)));
            heap.push((Reverse(height_map[i][cols - 1]), (i, cols - 1)));
            visited[i][0] = true;
            visited[i][cols - 1] = true;
        }

        for j in 1..cols - 1 {
            heap.push((Reverse(height_map[0][j]), (0, j)));
            heap.push((Reverse(height_map[rows - 1][j]), (rows - 1, j)));
            visited[0][j] = true;
            visited[rows - 1][j] = true;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((Reverse(height), (x, y))) = heap.pop() {
            for &(dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited[nx][ny] {
                        water_trapped += (height - height_map[nx][ny]).max(0);
                        heap.push((Reverse(height.max(height_map[nx][ny])), (nx, ny)));
                        visited[nx][ny] = true;
                    }
                }
            }
        }

        water_trapped
    }
}