impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut prefix_a = vec![0; a.len() + 1];
        let mut prefix_b = vec![0; b.len() + 1];
        let mut count = 0;
        let mut res = Vec::new();
        
        for idx in 0..a.len() {
            if a[idx] == b[idx] {
                count += 1;
                res.push(count);
                continue;
            }

            prefix_a[a[idx] as usize] = 1;
            prefix_b[b[idx] as usize] = 1;
            if prefix_a[b[idx] as usize] == 1 {
                count += 1;
            }

            if prefix_b[a[idx] as usize] == 1 {
                count += 1;
            }

            res.push(count);
        }

        res
    }
}