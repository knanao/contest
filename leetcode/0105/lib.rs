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

pub struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    // pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     let indeces = inorder
    //         .iter()
    //         .enumerate()
    //         .map(|(i, &v)| (v, i))
    //         .collect::<HashMap<_, _>>();
    //     Self::build(
    //         &mut preorder.iter(),
    //         &indeces,
    //         (0, preorder.len() as isize - 1),
    //     )
    // }

    // fn build(
    //     preorder: &mut std::slice::Iter<i32>,
    //     indeces: &HashMap<i32, usize>,
    //     range: (isize, isize),
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     if range.0 <= range.1 {
    //         if let Some(v) = preorder.next() {
    //             if let Some(i) = indeces.get(v) {
    //                 return Some(Rc::new(RefCell::new(TreeNode {
    //                     val: *v,
    //                     left: Self::build(preorder, indeces, (range.0, *i as isize - 1)),
    //                     right: Self::build(preorder, indeces, (*i as isize + 1, range.1)),
    //                 })));
    //             }
    //         }
    //     }
    //     None
    // }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::builder(&preorder, &inorder)
    }
    fn builder(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&f) = preorder.first() {
            let m = inorder.iter().position(|&e| e == f).unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: f,
                left: Self::builder(&preorder[1..(1 + m)], &inorder[0..m]),
                right: Self::builder(&preorder[(1 + m)..], &inorder[m + 1..]),
            })))
        } else {
            None
        }
    }
}
