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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&root, key)
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = root {
            let val = n.borrow().val;
            match val.cmp(&key) {
                std::cmp::Ordering::Greater => {
                    n.borrow_mut().left = Self::helper(&n.borrow().left, key);
                }
                std::cmp::Ordering::Less => {
                    n.borrow_mut().right = Self::helper(&n.borrow().right, key);
                }
                std::cmp::Ordering::Equal => {
                    if n.borrow().left.is_none() {
                        return n.borrow().right.clone();
                    }
                    if n.borrow().right.is_none() {
                        return n.borrow().left.clone();
                    }
                    let next = Self::search_next(&n.borrow().right);
                    if let Some(val) = next {
                        let r = Solution::helper(&n.borrow().right, val);
                        n.borrow_mut().val = val;
                        n.borrow_mut().right = r;
                    }
                }
            }
        }
        root.clone()
    }

    fn search_next(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                Solution::search_next(&n.borrow().left)
            } else {
                Some(n.borrow().val)
            }
        } else {
            None
        }
    }
}
