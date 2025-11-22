use std::collections::VecDeque;
impl Solution {
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        let mut dq = VecDeque::new();
        let mut v = vec![];
        dq.push_back(NestedInteger::List(nested_list));
        while !dq.is_empty() {
            let mut sum = 0;
            for _ in 0..dq.len() {
                let ni = dq.pop_front().unwrap();
                match ni {
                    NestedInteger::Int(val) => { sum += val; },
                    NestedInteger::List(vec) => {
                        for nni in vec {
                            dq.push_back(nni);
                        }
                    }
                }
            }
            v.push(sum);
        }
        
        let md = v.len() - 1;
        v.into_iter().enumerate().map(|(idx, val)| (md - idx + 1) as i32 * val).sum()
    }
}