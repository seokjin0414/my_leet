impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for val in arr.iter() {
            count = (count + (val % 2)) * (val % 2);
            if count >= 3 {
                return true;
            }
        }
        false
    }
}