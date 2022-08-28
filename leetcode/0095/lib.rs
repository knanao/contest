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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        Self::helper(0, n)
    }

    fn helper(s: i32, e: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ret = vec![None];
        if s > e {
            return ret;
        }

        for i in s..=e {
            let left = Self::helper(s, i - 1);
            let right = Self::helper(i + 1, e);
            for l in left {
                for r in right.clone() {
                    let mut c = TreeNode::new(i);
                    c.left = l.clone();
                    c.right = r;
                    ret.push(Some(Rc::new(RefCell::new(c))));
                }
            }
        }
        ret
    }
}
