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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        Self::helper(root.as_ref(), val);
        root
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val > val {
                let l = Self::helper(node.borrow().left.as_ref(), val);
                if l.is_some() {
                    node.borrow_mut().left = l;
                }
            } else {
                let r = Self::helper(node.borrow().right.as_ref(), val);
                if r.is_some() {
                    node.borrow_mut().right = r;
                }
            }
            None
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
