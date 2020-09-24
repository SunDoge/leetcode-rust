use leetcode_rust::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_depth(&root)
    }

    fn get_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            Self::get_depth(&node.clone().borrow().left)
                .max(Self::get_depth(&node.clone().borrow().right))
                + 1
        } else {
            0
        }
    }
}

fn some_rc_refcell(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(node)))
}

fn main() {
    let mut node3 = TreeNode::new(3);
    let node9 = TreeNode::new(9);
    let mut node20 = TreeNode::new(20);
    let node15 = TreeNode::new(15);
    let node7 = TreeNode::new(7);
    node3.left = some_rc_refcell(node9);
    node20.left = some_rc_refcell(node15);
    node20.right = some_rc_refcell(node7);
    node3.right = some_rc_refcell(node20);
    let root = some_rc_refcell(node3);

    assert_eq!(Solution::max_depth(root), 3);
}
