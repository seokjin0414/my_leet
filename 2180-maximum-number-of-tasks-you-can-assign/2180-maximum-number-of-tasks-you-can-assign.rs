use std::collections::VecDeque;

struct Problem {
    tasks: Vec<i32>,
    workers: Vec<i32>,
    pills: i32,
    strength: i32,
}

fn can_assign(p: &Problem, k: usize) -> bool {
    let mut selected = p.tasks[..k].to_vec();
    let mut available: VecDeque<_> = p.workers[p.workers.len() - k..].to_vec().into();
    let mut rem_pills = p.pills;

    for task in selected.iter().rev() {
        if let Some(&w) = available.back() {
            if w >= *task {
                available.pop_back();
                continue;
            }
        }
        if rem_pills == 0 {return false}

        if let Some(idx) = available.iter().position(|&w| w + p.strength >= *task) {
            available.remove(idx);
            rem_pills -= 1;
        } else {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
    let mut p = Problem { tasks, workers, pills, strength };
    p.tasks.sort_unstable();
    p.workers.sort_unstable();

    let (mut a, mut b) = (0, p.tasks.len().min(p.workers.len()));
    while a < b {
        let c = (a + b + 1) / 2;
        if can_assign(&p, c) {
            a = c;
        } else {
            b = c - 1;
        }
    }
    a as i32
    }
}