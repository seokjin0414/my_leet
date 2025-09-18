use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Entry {
    priority: i32,
    task_id: i32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then(self.task_id.cmp(&other.task_id))
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskManager {
    heap: BinaryHeap<Entry>,
    map: HashMap<i32, (i32, i32)>, // task_id -> (user_id, priority)
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut tm = TaskManager {
            heap: BinaryHeap::new(),
            map: HashMap::new(),
        };
        for t in tasks {
            tm.add(t[0], t[1], t[2]);
        }
        tm
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.map.insert(task_id, (user_id, priority));
        self.heap.push(Entry { priority, task_id });
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some((user, _)) = self.map.get_mut(&task_id) {
            *self.map.get_mut(&task_id).unwrap() = (*user, new_priority);
            self.heap.push(Entry { priority: new_priority, task_id });
        }
    }

    fn rmv(&mut self, task_id: i32) {
        self.map.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        while let Some(Entry { priority, task_id }) = self.heap.pop() {
            if let Some(&(user, p)) = self.map.get(&task_id) {
                if p == priority {
                    self.map.remove(&task_id);
                    return user;
                }
            }
        }
        -1
    }
}