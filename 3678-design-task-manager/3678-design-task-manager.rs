use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Task {
    priority: i32,
    task_id: i32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| self.task_id.cmp(&other.task_id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskManager {
    task_info: HashMap<i32, (i32, i32)>,
    heap: BinaryHeap<Task>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_info = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        for task in tasks {
            let user_id = task[0];
            let task_id = task[1];
            let priority = task[2];
            
            task_info.insert(task_id, (priority, user_id));
            heap.push(Task { priority, task_id });
        }
        
        Self { task_info, heap }
    }
    
    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_info.insert(task_id, (priority, user_id));
        self.heap.push(Task { priority, task_id });
    }
    
    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(info) = self.task_info.get_mut(&task_id) {
            info.0 = new_priority;
            self.heap.push(Task { priority: new_priority, task_id });
        }
    }
    
    fn rmv(&mut self, task_id: i32) {
        self.task_info.remove(&task_id);
    }
    
    fn exec_top(&mut self) -> i32 {
        while let Some(task) = self.heap.pop() {
            if let Some(&(priority, user_id)) = self.task_info.get(&task.task_id) {
                if priority == task.priority {
                    self.task_info.remove(&task.task_id);
                    return user_id;
                }
            }
        }
        -1
    }
}