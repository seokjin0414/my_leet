/*
// Definition for an Interval.
#[derive(PartialEq, Eq, Clone, Debug)]
struct Interval {
    pub start:i32,
    pub end:i32
}

impl Interval {
    #[inline]
    fn new(start:i32, end:i32) -> Self{
        Interval {
            start,
            end
        }
    }
}
*/

impl Solution {
    pub fn employee_free_time(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
        let mut data = vec![];

        for s in schedule {
            for p in s { data.push((p.start, p.end)); }
        } 
        data.sort();

        let mut last = data[0].1;     
        let mut ret = vec![];
        
        for d in data {
            if d.0 > last { ret.push(Interval::new(last, d.0)); }
            last = last.max(d.1); 
        }

        ret
    }
}