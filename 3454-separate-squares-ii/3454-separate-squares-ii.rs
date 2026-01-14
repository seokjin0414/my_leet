use std::collections::*;

#[derive(Clone, Default)]
struct Node {
    count: i32,
    occupied: i64,
    max_width: i64,
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let n = squares.len();
        let mut xs = Vec::with_capacity(n * 2);
        for s in &squares {
            xs.push(s[0] as i64);
            xs.push(s[0] as i64 + s[2] as i64);
        }
        xs.sort_unstable();
        xs.dedup();

        let m = xs.len();
        if m < 2 {
            return squares.first().map_or(0.0, |s| s[1] as f64);
        }

        let mut events = Vec::with_capacity(n * 2);
        for s in &squares {
            let x1 = s[0] as i64;
            let x2 = x1 + s[2] as i64;
            let y1 = s[1] as i64;
            let y2 = y1 + s[2] as i64;

            let i1 = xs.binary_search(&x1).unwrap();
            let i2 = xs.binary_search(&x2).unwrap();

            events.push((y1, 1, i1, i2));
            events.push((y2, -1, i1, i2));
        }
        events.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let num_leaves = m - 1;
        let mut tree = vec![Node::default(); 4 * num_leaves];
        Self::build(&mut tree, 1, 0, num_leaves - 1, &xs);

        let mut history = Vec::with_capacity(events.len());
        let mut total_area = 0;
        let mut prev_y = events[0].0;

        for (y, val, i1, i2) in events {
            let h = y - prev_y;
            if h > 0 {
                let w = tree[1].occupied;
                let area = w * h;
                total_area += area;
                history.push((total_area, prev_y, y, w));
            }

            if i1 < i2 {
                Self::update(&mut tree, 1, 0, num_leaves - 1, i1, i2 - 1, val);
            }
            prev_y = y;
        }

        let target = total_area as f64 / 2.0;
        let idx = history.partition_point(|x| (x.0 as f64) < target);

        if idx < history.len() {
            let (curr_total, y_start, _, w) = history[idx];
            let prev_total = if idx == 0 { 0 } else { history[idx - 1].0 };
            let needed = target - prev_total as f64;
            return y_start as f64 + needed / w as f64;
        }

        prev_y as f64
    }

    fn build(tree: &mut Vec<Node>, v: usize, tl: usize, tr: usize, xs: &[i64]) {
        if tl == tr {
            tree[v].max_width = xs[tl + 1] - xs[tl];
        } else {
            let tm = (tl + tr) / 2;
            Self::build(tree, 2 * v, tl, tm, xs);
            Self::build(tree, 2 * v + 1, tm + 1, tr, xs);
            tree[v].max_width = tree[2 * v].max_width + tree[2 * v + 1].max_width;
        }
    }

    fn update(tree: &mut Vec<Node>, v: usize, tl: usize, tr: usize, l: usize, r: usize, add: i32) {
        if l > r { return; }
        if l == tl && r == tr {
            tree[v].count += add;
        } else {
            let tm = (tl + tr) / 2;
            Self::update(tree, 2 * v, tl, tm, l, r.min(tm), add);
            Self::update(tree, 2 * v + 1, tm + 1, tr, l.max(tm + 1), r, add);
        }

        if tree[v].count > 0 {
            tree[v].occupied = tree[v].max_width;
        } else if tl != tr {
            tree[v].occupied = tree[2 * v].occupied + tree[2 * v + 1].occupied;
        } else {
            tree[v].occupied = 0;
        }
    }
}