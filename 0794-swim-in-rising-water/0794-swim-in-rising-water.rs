use std::collections::BinaryHeap;
impl Solution {
    pub fn swim_in_water(mut grid: Vec<Vec<i32>>) -> i32 {
        let (height, width) = (grid.len() as i32 - 1, grid[0].len() as i32 - 1);
        let mut heap = BinaryHeap::from([(-grid[0][0], 0, 0)]);
        while let Some((t, y, x)) = heap.pop() {
            if (y, x) == (height, width) {
                return -t;
            }
            for (dy, dx) in [
                (y > 0).then_some((y - 1, x)),
                (y < height).then_some((y + 1, x)),
                (x > 0).then_some((y, x - 1)),
                (x < width).then_some((y, x + 1)),
            ]
            .into_iter()
            .flatten()
            {
                let cell = &mut grid[dy as usize][dx as usize];
                if cell.is_negative() {
                    continue;
                }
                *cell = t.min(-*cell);
                heap.push((*cell, dy, dx));
            }
        }
        -1
    }
}