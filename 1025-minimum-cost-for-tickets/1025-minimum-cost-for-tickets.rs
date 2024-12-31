impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days.len();
        let mut dp = vec![0; n + 1];
        let mut last7_index = 0;
        let mut last30_index = 0;

        for i in 0..n {
            while days[last7_index] <= days[i] - 7 {
                last7_index += 1;
            }

            while days[last30_index] <= days[i] - 30 {
                last30_index += 1;
            }

            dp[i + 1] = dp[i] + costs[0];
            dp[i + 1] = dp[i + 1].min(dp[last7_index] + costs[1]);
            dp[i + 1] = dp[i + 1].min(dp[last30_index] + costs[2]);
        }

        dp[n]
    }
}