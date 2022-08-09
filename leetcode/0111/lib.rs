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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_min_depth(&root)
    }

    fn get_min_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        node.as_ref()
            .map(|node| node.borrow())
            .map(|node| {
                let (left, right) = (
                    Self::get_min_depth(&node.left),
                    Self::get_min_depth(&node.right),
                );
                if left != 0 && right != 0 {
                    return 1 + left.min(right);
                }
                if left == 0 || right == 0 {
                    return 1 + left + right;
                }
                0
            })
            .unwrap_or(0)
    }
}
