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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }
        if root2.is_none() {
            return root1;
        }
        let node1 = root1.unwrap();
        let node2 = root2.unwrap();
        let mut ret = TreeNode::new(node1.borrow().val + node2.borrow().val);
        ret.left = Self::merge_trees(node1.borrow().left.clone(), node2.borrow().left.clone());
        ret.right = Self::merge_trees(node1.borrow().right.clone(), node2.borrow().right.clone());
        return Some(Rc::new(RefCell::new(ret)));
    }
}
