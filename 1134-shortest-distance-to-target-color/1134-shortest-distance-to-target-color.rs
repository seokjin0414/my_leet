impl Solution {
  pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut data = vec![vec![], vec![], vec![]];
    for i in 0 .. colors.len() {
      data[(colors[i] - 1) as usize].push(i);
    }

    let mut res: Vec<i32> = Vec::with_capacity(queries.len());
    for i in 0 .. queries.len() {
      let (pos, c) = (queries[i][0] as usize, (queries[i][1] - 1) as usize);
      if data[c].is_empty() {
        res.push(-1);
        continue;
      }

      let r = data[c].binary_search(&pos);
      match r {
        Ok(_) => res.push(0),
        Err(p) => {
          if p == 0 {
            res.push((data[c][0] - pos) as i32);
          } else if p == data[c].len() {
            res.push((pos - data[c][p - 1]) as i32);
          } else {
            res.push((pos - data[c][p - 1]).min(data[c][p] - pos) as i32);
          }
        }
      }
    }

    return res;
  }
}