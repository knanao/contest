use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_max_depth(&root)
    }
    fn get_max_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        node.as_ref()
            .map(|node| node.borrow())
            .map(|node| 1 + Self::get_max_depth(&node.left).max(Self::get_max_depth(&node.right)))
            .unwrap_or(0)
    }
}
