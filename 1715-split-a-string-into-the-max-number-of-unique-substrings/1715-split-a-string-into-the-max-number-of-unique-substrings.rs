use std::collections::*;

struct Sub {
  result: usize,
  s: Vec<char>
}

impl Sub {
  fn dfs(&mut self, set: &mut HashSet<Vec<char>>, index: usize) {
    let size = self.s.len();

    if index == size {
      self.result = std::cmp::max(self.result, set.len());
      return
    }

    let mut vec = vec![];
    for i in index..size {
      vec.push(self.s[i]);

      if !set.contains(&vec) {
        set.insert(vec.clone());
        self.dfs(set, i+1);
        set.remove(&vec);
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