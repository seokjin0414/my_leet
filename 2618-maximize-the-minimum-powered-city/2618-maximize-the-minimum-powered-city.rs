impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let mut low = 0i64;
        let mut high = k as i64 + stations.iter().map(|&x| x as i64).sum::<i64>();

        while low < high {
            let mid = high + (low - high) / 2;
            if Self::check(&stations, mid, r as usize, k as i64) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        low
    }

    fn check(stations: &Vec<i32>, mid: i64, r: usize, mut k: i64) -> bool {
        let n = stations.len();
        let mut sum = 0i64;
        let mut ans = stations.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut sliding_window = vec![0i64; n];

        for i in 0..r.min(n) {
            sum += ans[i];
        }

        for i in 0..n {
            // Update sliding window sum
            if i + r < n {
                sum += ans[i + r];
            }
            if i > r {
                sum -= ans[i - r - 1];
            }

            if sum < mid {
                let needed = mid - sum;
                if needed > k {
                    return false;
                }

                if i + r < n {
                    ans[i + r] += needed;
                }
                k -= needed;
                sum = mid;
            }
        }

        true
    }
}