impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::with_capacity(spaces.len() + s.len());
        let mut prev = 0;
        for &space in &spaces {
            result.push_str(&s[prev as usize..space as usize]);
            result.push(' ');
            prev = space;
        }
        result.push_str(&s[prev as usize..]);
        result
    }
}