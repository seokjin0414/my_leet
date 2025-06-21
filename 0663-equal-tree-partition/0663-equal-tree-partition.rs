use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn check_equal_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let total = Self::sum(&root);

        if total % 2 != 0 {
            return false;
        }
        let target = total / 2;

        if let Some(node) = root {
            let node = node.borrow();
            Self::find(&node.left, target) || Self::find(&node.right, target)
        } else {
            false
        }
    }

    fn sum(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let mut n = n.borrow_mut();
            n.val += Self::sum(&n.left) + Self::sum(&n.right);
            n.val
        } else {
            0
        }
    }

    fn find(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
        if let Some(n) = node {
            let n = n.borrow();
            if n.val == target {
                return true;
            }
            Self::find(&n.left, target) || Self::find(&n.right, target)
        } else {
            false
        }
    }
}