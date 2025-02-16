fn backtrack(n: i32, x: &mut Vec<i32>, used: &mut Vec<bool>) -> Option<Vec<i32>> {
    if let Some(i) = x.iter().position(|&x|x==0) {
        for j in (1..=n as usize).rev() {
            if used[j] {continue}
            let k = if j == 1 {0} else {j};
            if x.get(i+k) == Some(&0) {
                x[i] = j as i32; x[i+k] = j as i32; used[j] = true;
                if let Some(r) = backtrack(n, x, used) {
                    return Some(r)
                }
                x[i] = 0; x[i+k] = 0; used[j] = false;
            }
        }
        return None
    } else {
        Some(x.to_vec())
    }
}


impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
       let mut x = vec![0; (2*n - 1) as usize];
       let mut used = vec![false; (n + 1) as usize];
       backtrack(n, &mut x, &mut used).unwrap()
    }
}