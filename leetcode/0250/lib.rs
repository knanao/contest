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
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut count = 0;
        Self::is_uni(root, &mut count);
        count
    }
    fn is_uni(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                *count += 1;
                return true;
            }

            let mut is_uni = true;
            if let Some(ref l) = n.left {
                is_uni = Self::is_uni(n.left.clone(), count) && is_uni && l.borrow().val == n.val;
            }

            if let Some(ref r) = n.right.clone() {
                is_uni = Self::is_uni(n.right.clone(), count) && is_uni && r.borrow().val == n.val;
            }

            if !is_uni {
                return false;
            }

            *count += 1;
            true
        } else {
            false
        }
    }
}
