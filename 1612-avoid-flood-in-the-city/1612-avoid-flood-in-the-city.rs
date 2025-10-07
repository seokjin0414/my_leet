impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut sunny_days = std::collections::BTreeSet::new();
        let mut last_full_days = std::collections::HashMap::with_capacity(rains.len());
        let mut result = vec![-1; rains.len()];

        for (day, lake) in rains.into_iter().enumerate() {
            if lake != 0 {
                if let Some(last_day) = last_full_days.get(&lake) {
                    if let Some(&sunny_day) = sunny_days.range(last_day + 1..).next() {
                        sunny_days.remove(&sunny_day);
                        result[sunny_day] = lake;
                    } else {
                        return Vec::new();
                    }
                }

                last_full_days.insert(lake, day);
            } else {
                sunny_days.insert(day);
                result[day] = 1;
            }
        }

        result
    }
}