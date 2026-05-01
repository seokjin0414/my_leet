use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
struct EmployeeSchedule {
    intervals: Vec<Interval>,
}

impl EmployeeSchedule {
    #[inline]
    fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }
}

impl From<Vec<Interval>> for EmployeeSchedule {
    #[inline]
    fn from(mut intervals: Vec<Interval>) -> Self {
        intervals.reverse();
        Self { intervals }
    }
}

impl PartialOrd for Interval {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl PartialOrd<Self> for EmployeeSchedule {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.intervals.last().partial_cmp(&other.intervals.last())
    }
}

impl Ord for EmployeeSchedule {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Less)
    }
}

impl Solution {
    pub fn employee_free_time(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
        let mut heap = schedule
            .into_iter()
            .filter(|intervals| !intervals.is_empty())
            .map(EmployeeSchedule::from)
            .map(Reverse)
            .collect::<BinaryHeap<_>>();

        let mut end = if let Some(Reverse(mut schedule)) = heap.pop() {
            let end = schedule.intervals.pop().unwrap().end;
            heap.push(Reverse(schedule));
            end
        } else {
            return vec![];
        };

        let mut free = vec![];
        while let Some(Reverse(mut schedule)) = heap.pop() {
            if schedule.is_empty() {
                continue;
            }
            let interval = schedule.intervals.pop().unwrap();
            if interval.start > end {
                free.push(Interval::new(end, interval.start));
            }
            end = end.max(interval.end);
            heap.push(Reverse(schedule));
        }

        free
    }
}