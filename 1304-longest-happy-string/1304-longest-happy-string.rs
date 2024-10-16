use std::collections::BinaryHeap;
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let abc = vec![(a, b'a'), (b, b'b'), (c, b'c')];
        let mut bh = BinaryHeap::new();
        
        for (bt, bb) in abc {
            if bt > 0 {
                bh.push((bt, bb));
            }
        }
        let mut v: Vec<u8> = vec![];

        while let Some((max_times, max_b)) = bh.pop() {
            if bh.is_empty() {
                if v.is_empty() || (*v.last().unwrap() != max_b) || (v[v.len() - 1] != v[v.len() - 2]) {
                    let t = max_times.min(2);
                    
                    for _ in 0..t {
                        v.push(max_b);
                    }
                }
                break;
            }
            
            let (sec_times, sec_b) = bh.pop().unwrap();
            let dd_mx = max_times.min(2);
            let arr = if v.is_empty() || *v.last().unwrap() != max_b {
                vec![(dd_mx, max_b), (1, sec_b)]
            } else {
                vec![(1, sec_b), (dd_mx, max_b)]
            };
            
            for sub_arr in arr {
                for _ in 0..sub_arr.0 {
                    v.push(sub_arr.1);
                }
            }

            if max_times > dd_mx {
                bh.push((max_times - dd_mx, max_b));
            }

            if sec_times > 1 {
                bh.push((sec_times - 1, sec_b));
            }
        }

        String::from_utf8(v).unwrap()
    }
}