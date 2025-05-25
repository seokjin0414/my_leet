use std::collections::HashMap;

impl Solution {
  pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut hash: HashMap<String, i32> = HashMap::new();
    let mut is_center: bool = false;
    let mut longest_length = 0;
  
    for word in &words {
      *hash.entry(word.clone()).or_insert(0) += 1;
    }
  
    for (word, count) in hash.clone() {
      if count == 0 {
        continue;
      }
      let rev: String = word.chars().rev().collect();

      if word == rev {
        let pair = count / 2;
        longest_length += 4 * pair;
        if count % 2 == 1 {
          is_center = true;
        }
      } else if hash.contains_key(&rev) {
        let min_pair = count.min(*hash.get(&rev).unwrap());

        longest_length += min_pair * 4;

        hash.insert(word.clone(), 0);
        hash.insert(rev.clone(), 0);
      }
    }
    if is_center {
      longest_length += 2;
    }

    longest_length 
  }
}