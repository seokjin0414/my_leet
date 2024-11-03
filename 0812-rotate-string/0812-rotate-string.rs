impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        for i in 0..s.len() {
            let x = s[i..s.len()].to_string() + & s[0..i].to_string();
            
            if x == goal { 
                return true
            }
        }

        return false
    }
}