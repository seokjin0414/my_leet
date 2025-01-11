use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if k > (s.len() as i32) {
            false
        } else {
            let mut char_count: HashMap<char, usize> = HashMap::new();

            for x in s.chars() {
                char_count.insert(x, if char_count.contains_key(&x) {
                    char_count.get(&x).unwrap() + 1
                } else {
                    1
                });
            }

            let mut odd = 0;

            for count in char_count.values() {
                odd = odd + (count % 2);
            }

            return (odd as i32) <= k;
        }
    }
}