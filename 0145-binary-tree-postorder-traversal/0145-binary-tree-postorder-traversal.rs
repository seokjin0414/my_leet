use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
   pub fn traverse(v: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(val) = node {
            let n = val.borrow();
            Self::traverse(v, &n.left);
            Self::traverse(v, &n.right);
            v.push(n.val);
        }
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Self::traverse(&mut v, &root);
        return v;
    }
}