impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let count_of_ones = data.iter().sum::<i32>();
        if count_of_ones <= 1 || count_of_ones == data.len() as i32 {
            return 0;
        }

        let mut min_swaps = i32::MAX;
        let mut current_zeros = 0;
        let mut left = 0;

        for right in 0..data.len() {
            if data[right] == 0 {
                current_zeros += 1;
            }

            if (right - left + 1) as i32 == count_of_ones {
                min_swaps = min_swaps.min(current_zeros);
                if data[left] == 0 {
                    current_zeros -= 1;
                }
                left += 1;
            }
        }
        min_swaps
    }
}