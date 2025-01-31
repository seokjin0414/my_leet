use std::collections::VecDeque;

impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut island_sizes = vec![0; 2];
        let mut island_id = 2;
        let mut max_island_size = 1;

        for row in 0..n {
            for col in 0..n {
                if grid[row][col] == 1 {
                    let size = Self::bfs_mark_island(&mut grid, row as i32, col as i32, island_id);
                    island_sizes.push(size);
                    max_island_size = max_island_size.max(size);
                    island_id += 1;
                }
            }
        }

        for row in 0..n {
            for col in 0..n {
                if grid[row][col] == 0 {
                    let mut adj_islands = Vec::new();

                    for &(dr, dc) in &Self::DIRECTIONS {
                        let nr = row as i32 + dr;
                        let nc = col as i32 + dc;
                        if Self::is_valid(nr, nc, n) {
                            let id = grid[nr as usize][nc as usize] as usize;
                            if id > 1 {
                                adj_islands.push(id);
                            }
                        }
                    }

                    if !adj_islands.is_empty() {
                        adj_islands.sort_unstable();
                        adj_islands.dedup();

                        let new_size = 1 + adj_islands.into_iter().map(|id| island_sizes[id]).sum::<i32>();
                        max_island_size = max_island_size.max(new_size);
                    }
                }
            }
        }

        max_island_size
    }

    fn bfs_mark_island(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, island_id: i32) -> i32 {
        let n = grid.len();
        let mut queue = VecDeque::new();
        queue.push_back((r, c));
        grid[r as usize][c as usize] = island_id;
        let mut size = 0;

        while let Some((row, col)) = queue.pop_front() {
            size += 1;

            for &(dr, dc) in &Self::DIRECTIONS {
                let nr = row + dr;
                let nc = col + dc;
                if Self::is_valid(nr, nc, n) && grid[nr as usize][nc as usize] == 1 {
                    grid[nr as usize][nc as usize] = island_id; 
                    queue.push_back((nr, nc));
                }
            }
        }

        size
    }

    fn is_valid(r: i32, c: i32, n: usize) -> bool {
        (0..n as i32).contains(&r) && (0..n as i32).contains(&c)
    }
}