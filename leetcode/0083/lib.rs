#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut h = head;
        let mut prev = h.as_mut().unwrap();
        while let Some(node) = prev.next.as_mut() {
            if node.val == prev.val {
                prev.next = node.next.take();
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }
        h
    }
}
