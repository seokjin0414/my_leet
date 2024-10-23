use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque; 
impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None
        } else {
            root.clone().unwrap().borrow_mut().val = 0;
        }
        
        let mut q = VecDeque::new();  //(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)
        q.push_back((root.clone().unwrap().borrow().left.clone(), root.clone().unwrap().borrow().right.clone()));

        while q.len() > 0 {
            let mut sum = 0;
            let mut level = vec![];

            for _ in 0..q.len() {
                match q.pop_front().unwrap() {
                    (Some(left), Some(right)) => {
                        sum += left.borrow().val;
                        sum += right.borrow().val;
                        q.push_back((left.borrow().left.clone(), left.borrow().right.clone()));
                        q.push_back((right.borrow().left.clone(), right.borrow().right.clone()));
                        level.push((Some(left), Some(right)));
                    }, 
                    (Some(left), None) => {
                        sum += left.borrow().val;
                        q.push_back((left.borrow().left.clone(), left.borrow().right.clone()));
                        level.push((Some(left), None));
                    }, 
                    (None, Some(right)) => {
                        sum += right.borrow().val;
                        q.push_back((right.borrow().left.clone(), right.borrow().right.clone()));
                        level.push((None, Some(right)));
                    }, 
                    (None, None) => {}
                }
            }

            for pair in level {
                match pair {
                    (Some(left), Some(right)) => {
                        let val = sum - left.borrow().val.clone() - right.borrow().val.clone();
                        left.borrow_mut().val = val;
                        right.borrow_mut().val = val;
                    }, 
                    (Some(left), None) => {
                        let val = sum - left.borrow().val.clone();
                        left.borrow_mut().val = val;
                    }, 
                    (None, Some(right)) => {
                        let val = sum - right.borrow().val.clone();
                        right.borrow_mut().val = val;
                    }, 
                    (None, None) => {}
                }
            }
        }
        root
    }
}