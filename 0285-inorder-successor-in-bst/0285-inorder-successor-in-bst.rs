use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    fn dfs1(node: &Node, list: &mut Vec<i32>) {
        if let Some(cur) = node {
            Self::dfs1(&cur.borrow().left.clone(), list);
            list.push(cur.borrow().val);
            Self::dfs1(&cur.borrow().right.clone(), list);
        }
    }

    fn dfs2(node: &Node, target: i32, ret: &mut Node) {
        if let Some(cur) = node {
            let cur_val = cur.borrow().val;
            if cur_val == target {
                *ret = node.clone();
                return;
            }
            Self::dfs2(&cur.borrow().left.clone(), target, ret);
            Self::dfs2(&cur.borrow().right.clone(), target, ret);
        }
    }
    
    pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut list = vec![];
        Self::dfs1(&root, &mut list);
        if let Some(cur) = p {
            let val = cur.borrow().val;
            let mut target = i32::MIN;            
            for (i, &l) in list.iter().enumerate() {
                if i < list.len() - 1 && l == val {
                    target = list[i + 1];
                }
            }

            if target == i32::MIN {
                return None;
            }
            
            let mut ret = None;
            Self::dfs2(&root, target, &mut ret);
            return ret;
        }
        None
    }
}