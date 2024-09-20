use std::cmp::{max};

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut n = 0;
        let len = candies.len();
        let mut vec = Vec::new();

        for i in (0..len) {
            n = max(candies[i], n);
        }

        n -= extra_candies;
        for i in (0..len) {
            vec.push(candies[i] >= n);
        }

        vec
    }
}