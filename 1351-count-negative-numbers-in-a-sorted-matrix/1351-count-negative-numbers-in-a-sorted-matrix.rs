impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n: i32 = grid.len().try_into().unwrap();
        let m: i32 = grid[0].len().try_into().unwrap();
        let mut count: i32 = 0;
        let mut i: i32 = n - 1;
        let mut j: i32 = 0;
        
        while 0 <= i {
            while j < m && 0 <= grid[usize::try_from(i).unwrap()][usize::try_from(j).unwrap()] {
                j += 1;
            }
            count += m - j;
            i -= 1;
        }

        count
    }
}