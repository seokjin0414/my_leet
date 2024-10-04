impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let sum: i64 = skill.iter().map(|&x| x as i64).sum();
        
        if sum % (n as i64 / 2) != 0 {
            return -1;
        }
        skill.sort_unstable();
        
        let t = skill[0] + skill[n - 1];
        let mut r = 0i64;
        
        for i in 0..n/2 {
            if skill[i] + skill[n - 1 - i] != t {
                return -1;
            }
            r += skill[i] as i64 * skill[n - 1 - i] as i64;
        }
        
        r
    }
}