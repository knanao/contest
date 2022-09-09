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
use std::collections::VecDeque;
use std::rc::Rc;
struct Codec;
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ret = "".to_string();
        Self::reserialize(root.as_ref(), &mut ret);
        ret
    }

    fn reserialize(root: Option<&Rc<RefCell<TreeNode>>>, s: &mut String) {
        if root.is_none() {
            s.push_str("null,");
        } else {
            let v = root.unwrap().borrow();
            s.push_str(format!("{},", v.val.to_string()).as_str());
            Self::reserialize(v.left.as_ref(), s);
            Self::reserialize(v.right.as_ref(), s);
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let sp: Vec<&str> = data.split(",").collect();
        let mut v = VecDeque::from(sp);
        Self::redeserialize(&mut v)
    }

    fn redeserialize(l: &mut VecDeque<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        let v = l.pop_front().unwrap();
        if v.eq("null") {
            return None;
        }

        let mut root = TreeNode::new(v.parse().unwrap());
        root.left = Self::redeserialize(l);
        root.right = Self::redeserialize(l);

        Some(Rc::new(RefCell::new(root)))
    }
}
