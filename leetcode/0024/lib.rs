// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return None;
        }
        let mut h = head;
        if let Some(node) = h.as_mut() {
            let tmp = node.val;
            if let Some(next) = node.next.as_mut() {
                node.val = next.val;
                next.val = tmp;
                next.next = Self::swap_pairs(next.clone().next);
            }
        }
        h
    }
}
