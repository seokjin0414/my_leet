impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut acc = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut ans = 0;
        let mut iter = nums.iter();
        
        iter.next_back(); // skip the last element.
        
        for &n in iter {
            acc -= n as i64 * 2;
            if acc <= 0 {
                ans += 1;
            }
        }
        ans
    }
}