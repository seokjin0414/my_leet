use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Brick {
    height: i32,
    x: usize, 
    y: usize,
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.height.cmp(&self.height)
    }
}

impl Solution {
    pub fn trap_rain_water(n_map: Vec<Vec<i32>>) -> i32 {
        let m = n_map.len() as usize;
        let n = n_map[0].len() as usize;
        let mut bricks: BinaryHeap<Brick> = BinaryHeap::new();
        let mut visited_bricks = vec![vec![false; n]; m];
        n_map.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, &height)| {
                if x == 0 || y == 0 || x == m - 1 || y == n - 1 {
                    visited_bricks[x][y] = true;
                    bricks.push(Brick {
                        height, x, y,
                    });
                }
            })
        });
        let mut trapped_water_quantity = 0;
        while let Some(Brick { height, x , y }) = bricks.pop() {
            let next_positions = vec![(x,y-1),(x,y+1),(x-1,y),(x+1,y)];
            for (r,c) in next_positions {
                if r < m && c < n && !visited_bricks[r][c] {
                    let curr_height = n_map[r][c];
                    if height > curr_height {
                        trapped_water_quantity += height - curr_height;
                    }
                    bricks.push(Brick{height: height.max(curr_height),x: r, y: c});
                    visited_bricks[r][c] = true;
                }
            }
        }
        trapped_water_quantity
    }
}