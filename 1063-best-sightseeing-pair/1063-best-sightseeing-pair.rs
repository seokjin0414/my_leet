impl Solution {
  pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut score = values[0];
    for i in 1..values.len() {
      score -= 1;
      if (score + values[i] > max) { max = score + values[i] }
      if (values[i] > score) { score = values[i] }
    }
    return max;
  }
}