use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut lakes  = HashMap::new();
        let mut days   = BTreeSet::new();
        let mut result = Vec::with_capacity(rains.len());

        for (today, lake) in rains.into_iter().enumerate() {
            if lake == 0 {
                days.insert(today);
                result.push(1);
            } else {
                if let Some(filled) = lakes.insert(lake, today) {
                    if let Some(&day) = days.range(filled..).next() {
                        result[day] = lake;
                        days.remove(&day);
                    } else {
                        return Vec::default();
                    }
                }
                result.push(-1);
            }
        }
        result
    }
}