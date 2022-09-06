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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_mirror(root.clone(), root.clone())
    }

    fn is_mirror(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        }
        if root1.is_none() || root2.is_none() {
            return false;
        }
        if let (Some(n1), Some(n2)) = (root1, root2) {
            return n1.borrow().val == n2.borrow().val
                && Self::is_mirror(n1.borrow().left.clone(), n2.borrow().right.clone())
                && Self::is_mirror(n1.borrow().right.clone(), n2.borrow().left.clone());
        }
        false
    }
}
