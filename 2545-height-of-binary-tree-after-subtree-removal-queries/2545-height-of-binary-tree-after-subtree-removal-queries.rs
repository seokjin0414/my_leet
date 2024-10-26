use std::rc::Rc;
use std::cell::RefCell;

type NodePtr = Rc<RefCell<TreeNode>>;
type NodeOpt = Option<NodePtr>;

impl Solution {
    pub fn tree_queries(root: NodeOpt, queries: Vec<i32>) -> Vec<i32> {
        let mut depths = Vec::new(); // Was a HashMap.

        get_depths(&root, &mut depths);

        let mut max_depths = vec![0; depths.len() + 1];

        get_max_depths(&root, -1, 0, &mut max_depths, &depths);

        queries.into_iter().map(|q| max_depths[q as usize]).collect()
    }
} 

fn get_depths(node_opt: &NodeOpt, depths: &mut Vec<i32>) -> i32 {
    if let Some(node_ptr) = node_opt {
        let node  = node_ptr.borrow();
        let value = node.val as usize;
        let depth = get_depths(&node.left, depths)
                    .max(get_depths(&node.right, depths));
        if value + 1 > depths.len() {
            depths.resize(value + 1, 0);
        }
        depths[value] = depth + 1;
        depth + 1
    } else {
        0
    }
}

fn get_max_depths(node_opt   : &NodeOpt, 
                  depth      : i32, 
                  max_depth  : i32, 
                  max_depths : &mut Vec<i32>,
                  depths     : &Vec<i32>) 
{
    if let Some(node_ptr) = node_opt {
        let node = node_ptr.borrow();

        let lt_depth = get_node_depth(&node.left, depths);
        let rt_depth = get_node_depth(&node.right, depths);

        max_depths[node.val as usize] = max_depth;
        
        get_max_depths(&node.left,
                       depth + 1, 
                       max_depth.max(depth + 1 + rt_depth),
                       max_depths,
                       depths);
        
        get_max_depths(&node.right,
                       depth + 1, 
                       max_depth.max(depth + 1 + lt_depth),
                       max_depths,
                       depths);
    }
}

fn get_node_depth(node_opt: &NodeOpt, depths: &Vec<i32>) -> i32 {
    node_opt.as_ref().map_or(0, |n| depths[n.borrow().val as usize])
}