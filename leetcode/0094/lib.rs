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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        Self::helper(root, &mut ret);
        ret
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        if let Some(node) = root {
            Self::helper(node.borrow().left.clone(), ret);
            ret.push(node.borrow().val);
            Self::helper(node.borrow().right.clone(), ret);
        }
    }
}
