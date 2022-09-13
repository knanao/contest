use std::cell::RefCell;
use std::rc::Rc;

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

struct BSTIterator {
    nodes: Vec<i32>,
    index: i32,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut i = BSTIterator {
            nodes: Vec::new(),
            index: -1,
        };
        Self::inorder(&mut i, root);
        i
    }

    fn inorder(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::inorder(self, node.borrow().left.clone());
            self.nodes.push(node.borrow().val);
            Self::inorder(self, node.borrow().right.clone());
        }
    }

    fn next(&mut self) -> i32 {
        self.index += 1;
        self.nodes[self.index as usize]
    }

    fn has_next(&self) -> bool {
        self.nodes.len() > (self.index + 1) as usize
    }
}
