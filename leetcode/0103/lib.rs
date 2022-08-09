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

/*
    1
   2 3
  4   5
*/

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut deque = VecDeque::new();
        deque.push_back((root, 0));
        let mut ret: Vec<Vec<i32>> = Vec::new();
        while let Some(d) = deque.pop_front() {
            if let Some(node) = d.0 {
                if let Some(v) = ret.get_mut(d.1) {
                    if d.1 % 2 == 0 {
                        v.push(node.borrow().val);
                    } else {
                        v.insert(0, node.borrow().val);
                    }
                } else {
                    ret.push(vec![node.borrow().val]);
                }
                deque.push_back((node.borrow().left.clone(), d.1 + 1));
                deque.push_back((node.borrow().right.clone(), d.1 + 1));
            }
        }
        ret
    }
}
