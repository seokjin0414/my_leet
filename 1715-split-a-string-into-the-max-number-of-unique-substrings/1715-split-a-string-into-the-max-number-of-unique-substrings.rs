use std::collections::*;

struct Sub {
  result: usize,
  s: Vec<char>
}

impl Sub {
  fn dfs(&mut self, set: &mut HashSet<Vec<char>>, size: usize) {
    let l = self.s.len();
    if size == l {
      self.result = std::cmp::max(self.result, set.len());
      return
    }

    let mut temp = vec![];
    for i in size..l {
      temp.push(self.s[i]);

      if !set.contains(&temp) {
        set.insert(temp.clone());
        self.dfs(set, i+1);
        set.remove(&temp);
      }
    }
  }
}

impl Solution {
  pub fn max_unique_split(s: String) -> i32 {
    let mut s = Sub {
        result: 0,
        s: s.chars().collect()
    };
    s.dfs(&mut HashSet::new(), 0);
    s.result as i32
  }
}