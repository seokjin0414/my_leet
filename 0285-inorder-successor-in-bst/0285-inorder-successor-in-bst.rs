use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut min_value_opt = None;
        let mut min_node = None;
        let target = p.as_ref().unwrap().borrow().val;

        let mut curr = root;

        while let Some(node_rc) = &curr {
            let node_ref = node_rc.borrow();
            let val = node_ref.val;
            let next = if val > target {
                if min_value_opt.is_none() || val < *min_value_opt.as_ref().unwrap() {
                    min_value_opt = Some(val);
                    min_node = curr.clone();
                }
                node_ref.left.clone()
            } else {
                node_ref.right.clone()
            };
            drop(node_ref);
            curr = next;
        }
        min_node
    }
}