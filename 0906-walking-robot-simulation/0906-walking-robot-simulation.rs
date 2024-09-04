use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let hash = obstacles.into_iter().collect::<HashSet<_>>();
        let dir = [0, 1, 0, -1, 0];
        let mut ans = 0;
        let mut pos = vec![0, 0];
        let mut d = 0;
        for &x in commands.iter() {
            match x {
                -1 => d = (d + 1) % 4,
                -2 => d = (d + 3) % 4,
                n => {
                    for _ in 0..n {
                        pos[0] += dir[d];
                        pos[1] += dir[d + 1];
                        if hash.contains(&pos) {
                            pos[0] -= dir[d];
                            pos[1] -= dir[d + 1];
                            break;
                        }
                    }
                    ans = ans.max(pos[0] * pos[0] + pos[1] * pos[1]);
                }
            }
        }
        ans
    }
}