impl Solution {
    pub fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (1, k as usize + 2);
        let mut ans = i32::MAX;
        let mut sol_found = false;

        let mut days = vec![0; bulbs.len() + 1];
        for i in 0..bulbs.len() {
            days[bulbs[i] as usize] = i + 1;
        }


        'outer: while right < days.len() {
            for i in (left + 1)..right {
                if days[i] < days[left] || days[i] < days[right] {
                    left = i;
                    right = i + k as usize + 1;
                    continue 'outer;
                }
            }

            ans = ans.min(days[left].max(days[right]) as i32);
            sol_found = true;
            left = right;
            right += k as usize + 1;
        }

        if sol_found {
            return ans;
        }

        -1
    }
}