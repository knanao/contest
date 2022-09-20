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
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            (Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone()))
                .abs()
                < 2
                && Self::is_balanced(node.borrow().left.clone())
                && Self::is_balanced(node.borrow().right.clone())
        } else {
            true
        }
    }

    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            1 + Self::height(node.borrow().left.clone())
                .max(Self::height(node.borrow().right.clone()))
        } else {
            -1
        }
    }
}
