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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(pn), Some(qn)) => {
                let pnb = pn.borrow();
                let qnb = qn.borrow();

                pnb.val == qnb.val
                    && Self::is_same_tree(pnb.left.clone(), qnb.left.clone())
                    && Self::is_same_tree(pnb.right.clone(), qnb.right.clone())
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
    let p = new_tree_node(
        1,
        new_tree_node(2, None, None),
        new_tree_node(1, None, None),
    );
    let q = new_tree_node(
        1,
        new_tree_node(1, None, None),
        new_tree_node(2, None, None),
    );

    assert_eq!(Solution::is_same_tree(p, q), false);
}
