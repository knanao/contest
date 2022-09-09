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
        Self::dfs(root.as_ref(), p.as_ref(), q.as_ref())
    }

    fn dfs(
        root: Option<&Rc<RefCell<TreeNode>>>,
        p: Option<&Rc<RefCell<TreeNode>>>,
        q: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root.cloned();
        }

        let left = Self::dfs(root.unwrap().borrow().left.as_ref(), p, q);
        let right = Self::dfs(root.unwrap().borrow().right.as_ref(), p, q);

        if left.is_some() && right.is_some() {
            return root.cloned();
        }

        if left.is_some() {
            return left;
        }

        if right.is_some() {
            return right;
        }
        None
    }
}
