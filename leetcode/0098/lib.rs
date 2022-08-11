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

struct Solution;

/*   5
    4 6
     3 7
*/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid(root.as_ref(), std::i64::MIN, std::i64::MAX)
    }

    fn is_valid(root: Option<&Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(r) = root {
            if r.borrow().val as i64 <= min || r.borrow().val as i64 >= max || min >= max {
                return false;
            } else {
                return Self::is_valid(r.borrow().left.as_ref(), min, r.borrow().val as i64)
                    && Self::is_valid(r.borrow().right.as_ref(), r.borrow().val as i64, max);
            }
        }
        true
    }
}
