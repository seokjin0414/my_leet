impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let (mut sum, mut min_abs, mut count_neg) = (0i64, i32::MAX, 0);
        for row in matrix.iter() {
            for &val in row.iter() {
                if val < 0 {
                    count_neg += 1;
                }
                min_abs = min_abs.min(val.abs());
                sum += val.abs() as i64;
            }
        }
        if count_neg % 2 == 1 {
            sum -= (2 * min_abs) as i64;
        }
        sum
    }
}