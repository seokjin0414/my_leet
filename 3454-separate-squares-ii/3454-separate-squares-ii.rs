struct SegTree {
    count: Vec<i32>,
    len: Vec<i64>,
    xs: Vec<i64>,
}

impl SegTree {
    fn new(xs: Vec<i64>) -> Self {
        SegTree {
            count: vec![0; xs.len() * 4],
            len: vec![0; xs.len() * 4],
            xs,
        }
    }

    fn update_and_query(
        &mut self,
        idx: usize,
        start: usize,
        end: usize,
        left: i64,
        right: i64,
        val: i64,
    ) -> i64 {
        let start_x = self.xs[start];
        let end_x = self.xs[end + 1];

        if start_x >= right || end_x <= left {
            return self.len[idx];
        }

        if start_x >= left && end_x <= right {
            self.count[idx] += val as i32;
        } else if start != end {
            self.update_and_query(idx * 2, start, (start + end) / 2, left, right, val);
            self.update_and_query(idx * 2 + 1, (start + end) / 2 + 1, end, left, right, val);
        }

        if self.count[idx] > 0 {
            self.len[idx] = end_x - start_x;
        } else if start != end {
            self.len[idx] = self.len[idx * 2] + self.len[idx * 2 + 1];
        } else {
            self.len[idx] = 0;
        }

        self.len[idx]
    }
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut events = Vec::with_capacity(squares.len() * 2);
        let mut xs = Vec::with_capacity(squares.len() * 2);

        for square in squares {
            let (x, y, l) = (square[0] as i64, square[1] as i64, square[2] as i64);

            xs.push(x);
            xs.push(x + l);
            events.push((y, x, x + l, 1));
            events.push((y + l, x, x + l, -1));
        }

        events.sort_unstable_by_key(|e| e.0);
        xs.sort_unstable();
        xs.dedup();

        let n = xs.len() - 1;

        let mut seg_tree = SegTree::new(xs);

        let mut area_sum = 0;
        let mut cur_y = 0;
        let mut cur_l = 0;

        let mut prefix_area_y = Vec::with_capacity(events.len());

        for &(y, left, right, val) in &events {
            area_sum += (y - cur_y) * cur_l;
            cur_y = y;
            cur_l = seg_tree.update_and_query(1, 0, n - 1, left, right, val);

            prefix_area_y.push((area_sum, cur_y, cur_l));
        }

        let idx = prefix_area_y.partition_point(|x| x.0 * 2 < area_sum);
        let (area, y, l) = prefix_area_y[idx - 1];

        y as f64 + (area_sum as f64 / 2.0 - area as f64) / l as f64
    }
}