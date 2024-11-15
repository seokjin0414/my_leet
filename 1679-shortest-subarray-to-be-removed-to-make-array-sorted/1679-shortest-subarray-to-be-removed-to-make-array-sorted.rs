use std::cmp;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let n = arr.len();
        let mut right = arr.len() - 1;
        while left + 1 < n && arr[left] <= arr[left + 1] {
            left += 1;
        }
        if left == n - 1 {
            return 0;
        }
        while right - 1 >= left && arr[right] >= arr[right - 1] {
            right -= 1;
        }
        let mut ans = cmp::max(n - right, left+1);
        let mut i = 0;
        let mut j = right;
        while i <= left && j < n {
            if arr[i] <= arr[j] {
                ans = cmp::max(ans, i + 1 + ( n - j));
                i += 1;
            } else {
                j += 1;
            }
        }
        (n - ans) as i32
    }
}