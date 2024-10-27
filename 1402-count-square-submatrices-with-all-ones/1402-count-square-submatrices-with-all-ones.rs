impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = matrix.clone();
        let mut count: i32 = 0;

        count += dp[0].iter().fold(0,|acc,x|acc+x );
        count += dp.iter().map(|row|row[0]).skip(1).fold(0,|acc,x|acc+x );

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 1 {
                    dp[i][j] = 1+dp[i-1][j-1].min(dp[i][j-1].min(dp[i-1][j]));
                    count += dp[i][j]; 
                }
            }
        }
        
        count
    }
}