impl Solution {
    fn dfs(i: i32, j: i32, n: &i32, grid: &mut Vec<Vec<i32>>) -> (i64, i32) {
        let d: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut s: i64 = grid[i as usize][j as usize] as i64;
        let mut c: i32 = 1;
        grid[i as usize][j as usize] = 0;
        for (di, dj) in d.iter() {
            let ni: i32 = i+di;
            let nj: i32 = j+dj;
            if ni >= 0 && ni < *n && nj >= 0 && nj < *n && grid[ni as usize][nj as usize] > 0 {
                let (v, _c) = Self::dfs(ni, nj, n, grid);
                s += v;
                c += _c; 
            }
        }
        (s, c)
    }

    pub fn sum_remoteness(mut grid: Vec<Vec<i32>>) -> i64 {
        let n: i32 = grid.len() as i32;
        let t: i64 = grid.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |racc, &v| if v > 0 { racc + v as i64 } else { racc })
        });
        let mut ans: i64 = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i as usize][j as usize] > 0 {
                    let (v, c) = Self::dfs(i, j, &n, &mut grid);
                    ans += (t-v)*c as i64;
                }
            }
        }
        ans
    }
}