impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {        
        fn dfs(x: i32, n_ref: &i32, v: &mut Vec<i32>) {
            for i in 0..10 {
                let tmp = x * 10 + i; 

                if tmp > *n_ref || tmp == 0 {
                    continue; 
                }
                v.push(tmp);
                dfs(tmp, n_ref, v);
            } 
        }

        let mut v = Vec::with_capacity(n as usize);
        dfs(0, &n, &mut v);
        
        v
    }
}