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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(root, p.as_ref(), q.as_ref())
    }

    fn helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<&Rc<RefCell<TreeNode>>>,
        q: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let p_v = p.unwrap().borrow().val;
            let q_v = q.unwrap().borrow().val;

            if node.borrow().val > p_v && node.borrow().val > q_v {
                Self::helper(node.borrow().left.clone(), p, q);
            } else if node.borrow().val < p_v && node.borrow().val < q_v {
                Self::helper(node.borrow().right.clone(), p, q);
            }
        }
        root.clone()
    }
}
