impl Solution {
    pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp =  vec![vec![0; n]; m];
        
        for i in 0 .. m {
            let mut sk = vec![];
            let mut cnt = 0;
            
            for j in 0 .. n {
                if grid[i][j] == '0' { sk.push(j); }
                if grid[i][j] == 'E' { cnt += 1; }
                
                if grid[i][j] == 'W' || j == n - 1 {
                    while let Some(k) = sk.pop() {
                        dp[i][k] += cnt; 
                    }
                    cnt = 0;
                }
            }
        }
        
        for j in 0 .. n {
            let mut sk = vec![];
            let mut cnt = 0;
            
            for i in 0 .. m {
                if grid[i][j] == '0' { sk.push(i); }
                if grid[i][j] == 'E' { cnt += 1; }
                
                if grid[i][j] == 'W' || i == m - 1 {
                    while let Some(k) = sk.pop() {
                        dp[k][j] += cnt; 
                    }
                    cnt = 0;
                }
            }
        }
        
        let mut ret = 0;
        
        for d in dp {
            ret = ret.max(d.into_iter().max().unwrap());
        }
        
        ret
    }
}