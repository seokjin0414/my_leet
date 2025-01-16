impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        match [nums1.len() & 1 == 0, nums2.len() & 1 == 0] {
            [true, true] => Some(0),
            [true, false] => nums1.into_iter().reduce(|r, n| r ^ n),
            [false, true] => nums2.into_iter().reduce(|r, n| r ^ n),
            [false, false] => nums1.into_iter().chain(nums2).reduce(|r, n| r ^ n),
        }
        .unwrap()
    }
}