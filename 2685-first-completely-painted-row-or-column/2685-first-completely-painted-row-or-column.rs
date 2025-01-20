impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat.first().map_or(0, |r| r.len()));
        
        let mut val2pos = vec![(0, 0); m * n + 1];

        let mut row_counts = vec![0; m + 1];
        let mut col_counts = vec![0; n + 1];

        for (r, row) in mat.into_iter().enumerate() {
            for (c, val) in row.into_iter().enumerate() {
                val2pos[val as usize] = (r, c);
            }
        }
        for (i, val) in arr.into_iter().enumerate() {
            let (row, col) = val2pos[val as usize];

            row_counts[row] += 1;
            col_counts[col] += 1;

            if row_counts[row] == n || col_counts[col] == m {
                    
                return i as i32;
            }
        }
        -1
    }
}