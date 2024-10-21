use std::collections::*;

struct Cal {
  result: usize,
  s: Vec<char>
}

impl Cal {
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
    let mut cal = Cal {
        result: 0,
        s: s.chars().collect()
    };
    cal.dfs(&mut HashSet::new(), 0);
    cal.result as i32
  }
}