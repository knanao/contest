use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut deque = VecDeque::new();
        if let Some(r) = root {
            deque.push_back(r);
        }
        let mut ret = Vec::new();
        while !deque.is_empty() {
            let mut row = Vec::new();
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front() {
                    row.push(node.borrow().val);
                    if let Some(l) = node.borrow_mut().left.take() {
                        deque.push_back(l);
                    }
                    if let Some(r) = node.borrow_mut().right.take() {
                        deque.push_back(r);
                    }
                }
            }
            ret.push(row);
        }
        ret
    }
}
