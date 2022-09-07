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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::builder(&inorder, &postorder)
    }

    fn builder(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&l) = postorder.last() {
            let m = inorder.iter().position(|&e| e == l).unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: l,
                left: Self::builder(&inorder[0..m], &postorder[0..m]),
                right: Self::builder(&inorder[m + 1..], &postorder[m..postorder.len() - 1]),
            })))
        } else {
            None
        }
    }
}
