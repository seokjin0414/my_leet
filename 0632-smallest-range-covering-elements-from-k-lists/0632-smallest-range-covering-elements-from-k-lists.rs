impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v: Vec<(i32, usize)> = Vec::new();
        let K: usize = nums.len();
        for k in 0..K {
            for a in &nums[k] {
                v.push((*a, k));
            }
        }
        v.sort_by_key(|x| x.0);

        let mut count: Vec<usize> = vec![0; K];
        let mut i: usize = 0;
        let mut k: usize = 0;

        let mut lb: i32 = -100001;
        let mut ub: i32 = 100001;
        for j in 0..v.len() {
            count[v[j].1] += 1;
            if count[v[j].1] == 1 { k += 1; }
            if k == K {
                while i <= j && count[v[i].1] > 1 {
                    count[v[i].1] -= 1;
                    i += 1;
                }
                if v[j].0 - v[i].0 < ub - lb {
                    ub = v[j].0;
                    lb = v[i].0;
                }
            }
        }
        vec![lb, ub]
    }
}