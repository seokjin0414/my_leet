impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() & 1usize == 1 {
            return false;
        }

        let mut reversable_to_left : i32 = 0;
        let mut reversable_to_right : i32 = 0;

        for (c, is_locked) in s.chars().zip(locked.chars()) {
            reversable_to_left += (is_locked == '0' || c == '(') as i32;
            reversable_to_left -= (is_locked != '0' && c == ')') as i32;
            if reversable_to_left < 0 {
                return false;
            }
        }

        for (c, is_locked) in s.chars().rev().zip(locked.chars().rev()) {
            reversable_to_right += (is_locked == '0' || c == ')') as i32;
            reversable_to_right -= (is_locked != '0' && c == '(') as i32;
            if reversable_to_right < 0 {
                return false;
            }
        }
        return true;
    }
}