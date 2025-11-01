use std::{collections::BinaryHeap, cmp::Reverse};

impl Solution {
    pub fn maximum_number_of_ones(width: i32, height: i32, side_length: i32, max_ones: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for i in 0..side_length {
            for j in 0..side_length {
                heap.push(Reverse(((width - 1 - i) / side_length + 1) * ((height - 1 - j) / side_length + 1)));
                if heap.len() > max_ones as usize { heap.pop(); }
            }
        }
        heap.into_iter().map(|Reverse(ones)| ones).sum()
    }
}