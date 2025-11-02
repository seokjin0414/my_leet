impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut mat = vec![vec![0 as i32; n as usize]; m as usize];

        for guard in &guards {
            mat[guard[0] as usize][guard[1] as usize] = 2;
        }

        for wall in &walls {
            mat[wall[0] as usize][wall[1] as usize] = 3;
        }

        let mut ans = m * n - (guards.len() as i32) - (walls.len() as i32);
        for guard in &guards {
            ans -= Self::propagate(guard[0], guard[1], &mut mat);
        }

        ans
    }

    pub fn propagate(r: i32, c: i32, mat: &mut Vec<Vec<i32>>) -> i32 {
        let mut res = 0;

        let mut dr = 1;
        while ((r + dr) as usize) < mat.len() {
            if mat[(r + dr) as usize][c as usize] == 0 {
                mat[(r + dr) as usize][c as usize] = 1;
                res += 1;
            } else if mat[(r + dr) as usize][c as usize] > 1 {
                break;
            }
            dr += 1;
        }
        dr = -1;
        while r + dr >= 0 {
            if mat[(r + dr) as usize][c as usize] == 0 {
                mat[(r + dr) as usize][c as usize] = 1;
                res += 1;
            } else if mat[(r + dr) as usize][c as usize] > 1 {
                break;
            }
            dr -= 1;
        }

        let mut dc = 1;
        while ((c + dc) as usize) < mat[0].len() {
            if mat[r as usize][(c + dc) as usize] == 0 {
                mat[r as usize][(c + dc) as usize] = 1;
                res += 1;
            } else if mat[r as usize][(c + dc) as usize] > 1 {
                break;
            }
            dc += 1;
        }
        dc = -1;
        while c + dc >= 0 {
            if mat[r as usize][(c + dc) as usize] == 0 {
                mat[r as usize][(c + dc) as usize] = 1;
                res += 1;
            } else if mat[r as usize][(c + dc) as usize] > 1 {
                break;
            }
            dc -= 1;
        }

        res
    }
}