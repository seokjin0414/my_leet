impl Solution {
  pub fn minimum_steps(s: String) -> i64 {
    let (mut res, mut last_zero, s) = (0, 0, s.as_bytes());

    for i in 0 .. s.len() {
      if s[i] == 48 {
        res += i - last_zero;
        last_zero += 1;
      }
    }

    return res as i64;
  }
}