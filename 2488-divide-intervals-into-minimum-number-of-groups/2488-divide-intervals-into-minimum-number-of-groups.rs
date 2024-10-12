impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut f: Vec<i32> = Vec::with_capacity(intervals.len());
        let mut l: Vec<i32> = Vec::with_capacity(intervals.len());

        for interval in &intervals {
            f.push(interval[0]);
            l.push(interval[1]);
        }

        f.sort_unstable();
        l.sort_unstable();

        let mut n = 0;
        let mut max = 0;
        let mut f_idx = 0;
        let mut l_idx = 0;

        while f_idx < intervals.len() {
            if f[f_idx] <= l[l_idx] {
                n += 1;
                f_idx += 1;
                max = max.max(n);
            } else {
                n -= 1;
                l_idx += 1;
            }
        }

        max
    }
}