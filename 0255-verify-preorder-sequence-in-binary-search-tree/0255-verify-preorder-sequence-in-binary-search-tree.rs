use std::collections::VecDeque;

impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        let mut lookup = VecDeque::new();
        let mut max_until_now = i32::MIN;
        lookup.push_back(preorder[0]);

        for node in preorder.iter().skip(1) {
            while !lookup.is_empty() && lookup.back().unwrap() < node {
                let back = lookup.pop_back().unwrap();
                max_until_now = max_until_now.max(back);
            }
            if *node < max_until_now {
                return false;
            }
            lookup.push_back(*node);
        }
        true
    }
}