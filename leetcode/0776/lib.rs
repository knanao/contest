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
    pub fn split_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(v) = root {
            let mut node = v.borrow_mut();
            if node.val <= target {
                let mut bns = Self::split_bst(node.right.clone(), target);
                node.right = bns[0].clone();
                bns[0] = Some(v.clone());
                return bns;
            } else {
                let mut bns = Self::split_bst(node.left.clone(), target);
                node.left = bns[1].clone();
                bns[1] = Some(v.clone());
                return bns;
            }
        }
        return vec![None, None];
    }
}
