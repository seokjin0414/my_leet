use std::collections::VecDeque;

impl Solution {
    
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights.first().map_or(0, |r| r.len());

        let mut pac_queue = VecDeque::new();
        let mut atl_queue = VecDeque::new();

        let mut pac_reached = vec![vec![false; n]; m];
        let mut atl_reached = vec![vec![false; n]; m];

        for r in 0..m {
            pac_reached[r][0] = true;
            pac_queue.push_back((r, 0));

            atl_reached[r][n - 1] = true;
            atl_queue.push_back((r, n - 1));
        }

        for c in 0..n {
            pac_reached[0][c] = true;
            pac_queue.push_back((0, c));

            atl_reached[m - 1][c] = true;
            atl_queue.push_back((m - 1, c));
        }

        for (mut queue, reached) in [(pac_queue, &mut pac_reached), 
                                     (atl_queue, &mut atl_reached)] {
            while let Some((r1, c1)) = queue.pop_front() {
                let adjacent = [(r1 + 1, c1), (r1, c1 + 1),
                                (r1.wrapping_sub(1), c1),
                                (r1, c1.wrapping_sub(1))];

                for (r2, c2) in adjacent {
                    if r2 < m && c2 < n 
                        && !reached[r2][c2]
                        && heights[r2][c2] >= heights[r1][c1] {

                        reached[r2][c2] = true;
                        queue.push_back((r2, c2));
                    }
                }
            }
        }
        let mut result = Vec::new();

        for r in 0..m {
            for c in 0..n {
                if pac_reached[r][c] && atl_reached[r][c] {
                    result.push(vec![r as i32, c as i32]);
                }
            }
        }
        result
    }
}