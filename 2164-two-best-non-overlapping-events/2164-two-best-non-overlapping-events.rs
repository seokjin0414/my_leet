impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
            use std::cmp::Reverse;

    #[derive(Copy,Clone,Debug,Hash,Eq,PartialEq)]
    struct Ev {
        idx: usize,
        start: i32,
        end: i32,
        val: i32
    }

    let mut events = events.into_iter().enumerate().map(|(idx,v)| {
        let &[start, end, val] = &v[..] else { unreachable!(); };
        Ev { idx, start, end, val }
    }).collect::<Vec<Ev>>();
    events.sort_by_key(|x| Reverse((x.val, x.end-x.start)));

    let cap = events.iter().copied().map(|e| e.end).max().unwrap()
            - events.iter().copied().map(|e| e.start).min().unwrap()
            + 1;

    let mut by_len = events.clone();
    by_len.sort_by_key(|e| e.end-e.start);

    let mut out = events.iter().copied().map(|e|e.val).max().unwrap();

    for e in events {
        if 2*e.val <= out {
             break;
        }
        
        let cap = cap - (e.end-e.start+1);
       
        for e2 in by_len.iter().copied() {           
            if e2.end-e2.start+1 > cap { break; }
            if e2.val + e.val <= out { continue; }
            if e2.idx == e.idx { continue; }

            if e2.start > e.end || e2.end < e.start {
                out = e.val + e2.val;
            }
        }
    }

    out
    }
}