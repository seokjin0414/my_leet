impl Solution {
  fn update_counts(cnts: &mut Vec<i32>, mut val: i32, delta: i32) {
    let mut pos = 0;
    while val != 0 {
      if val % 2 == 1 {
        cnts[pos] += delta;
      }
      val /= 2;
      pos += 1;
    }
  }

  fn counts_to_num(cnts: &Vec<i32>) -> i32 {
    let (mut res, mut mult) = (0, 1);
    for i in 0 .. 30 {
      if cnts[i] > 0 {
        res += mult;
      }
      mult *= 2;
    }
    return res
  }

  pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    if k == 0 {
      return 1
    }
    
    let (mut l, mut res, mut cnt) = (0, usize::MAX, vec![0; 30]);
    for r in 0 .. nums.len() {
      Self::update_counts(&mut cnt, nums[r], 1);
      while Self::counts_to_num(&cnt) >= k {
        res = res.min(r - l + 1);
        Self::update_counts(&mut cnt, nums[l], -1);
        l += 1;
      }
    }

    if res == usize::MAX {
      return -1
    }
    return res as i32
  }
}