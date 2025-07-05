use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32, i32, i32) {
            if let Some(n) = node {
                let n = n.borrow();
                let (l_bst, l_min, l_max, l_size) = dfs(n.left.clone());
                let (r_bst, r_min, r_max, r_size) = dfs(n.right.clone());

                if l_bst && r_bst && l_max < n.val && n.val < r_min {
                    let size = l_size + r_size + 1;
                    let min_val = if l_min != i32::MAX { l_min } else { n.val };
                    let max_val = if r_max != i32::MIN { r_max } else { n.val };
                    return (true, min_val, max_val, size);
                }
                (false, 0, 0, l_size.max(r_size))
            } else {
                (true, i32::MAX, i32::MIN, 0)
            }
        }

        let (_, _, _, max_bst_size) = dfs(root);
        max_bst_size
    }
}