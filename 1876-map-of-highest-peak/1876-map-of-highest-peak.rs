impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = is_water.len();    
        let cols = is_water[0].len();

        let mut h = vec![vec![false; cols]; rows];
        let mut cur = vec![];

        for r in 0..rows {
            for c in 0..cols {
                if is_water[r][c] == 1 {
                    h[r][c] = true;
                    cur.push((r, c));
                }
            }
        }
        let dir = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut gen = vec![];
        let mut level = 0;
        
        while !cur.is_empty() {
            for &(r, c) in &cur {
                is_water[r][c] = level;
                for (dr, dc) in dir {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if (0..rows as i32).contains(&nr) && (0..cols as i32).contains(&nc) {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !h[nr][nc] {
                            h[nr][nc] = true;
                            gen.push((nr, nc));
                        }
                    }
                }
            }

            level += 1;
            cur.clear();
            std::mem::swap(&mut cur, &mut gen);
        }

        is_water
    }
}