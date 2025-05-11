use std::collections::HashMap;
#[derive(Debug, Default)]
struct Logger {
    map: HashMap<String, i32>,
}

impl Logger {

    fn new() -> Self {
        Default::default()
    }
    
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(&v) = self.map.get(&message) {
            if timestamp < v {
                return false;
            }
        }
        self.map.insert(message, timestamp + 10);
        true
    }
}