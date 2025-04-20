use std::collections::HashMap;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut longest = 0;
        let mut map = [(0, -1)].into_iter().collect::<HashMap<i32,i32>>();
        nums
            .into_iter()
            .enumerate()
            .fold(0, |mut sum, (index, num)| {
                sum += num;

                if num == k {
                    longest = longest.max(1);
                }
                match map.get(&(sum - k)) {
                    Some(previous_index) => {
                        longest = longest.max(index as i32 - previous_index);
                    },
                    None => (),
                }
                map.entry(sum).or_insert(index as i32);

                sum
            });

        longest as i32
    }
}