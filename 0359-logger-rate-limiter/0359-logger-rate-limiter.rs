use std::collections::{HashSet, VecDeque};

struct Logger {
    msg_set: HashSet<String>,
    msg_queue: VecDeque<(String, i32)>,
}

impl Logger {

    fn new() -> Self {
        Self {
            msg_set: HashSet::new(),
            msg_queue: VecDeque::new(),
        }
    }
    
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        while let Some((msg, ts)) = self.msg_queue.front() {
            if timestamp - ts >= 10 {
                if let Some((old_msg, _)) = self.msg_queue.pop_front() {
                    self.msg_set.remove(&old_msg);
                }
            } else {
                break;
            }
        }

        if !self.msg_set.contains(&message) {
            self.msg_set.insert(message.clone());
            self.msg_queue.push_back((message, timestamp));
            true
        } else {
            false
        }
    }
}