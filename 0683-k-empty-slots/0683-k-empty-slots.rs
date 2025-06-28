use std::collections::BTreeSet;

impl Solution {
    pub fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
        let mut s = BTreeSet::<i32>::new();
        
        for i in 0..bulbs.len() {
            let a = bulbs[i];
            
            if let Some(b) = s.range(a..).next() {
                if *b - a == k + 1 { return i as i32 + 1 }
            }
            
            if let Some(b) = s.range(..a).next_back() {
                if a - *b == k + 1 { return i as i32 + 1 }
            }
            s.insert(a);
        } 
        
        -1
    }
}