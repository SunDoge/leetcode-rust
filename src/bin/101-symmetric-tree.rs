// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            Self::compare(r.borrow().left.clone(), r.borrow().right.clone())
        } else {
            true
        }
    }

    fn compare(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                let lb = l.borrow();
                let rb = r.borrow();

                lb.val == rb.val
                    && Self::compare(lb.left.clone(), rb.right.clone())
                    && Self::compare(lb.right.clone(), rb.left.clone())
            }
            _ => false,
        }
    }
}

fn new_tree_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn main() {
    let root = new_tree_node(
        1,
        new_tree_node(
            2,
            new_tree_node(3, None, None),
            new_tree_node(4, None, None),
        ),
        new_tree_node(
            2,
            new_tree_node(4, None, None),
            new_tree_node(3, None, None),
        ),
    );

    assert_eq!(Solution::is_symmetric(root), true);
}
