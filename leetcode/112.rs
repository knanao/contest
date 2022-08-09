use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

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

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::is_equal(&root, target_sum)
    }

    fn is_equal(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(v) = root {
            let node = v.borrow();
            if node.left.is_none() && node.right.is_none() {
                return sum == node.val;
            }
            return Self::is_equal(&node.left, sum - node.val)
                || Self::is_equal(&node.right, sum - node.val);
        }
        false
    }
}
