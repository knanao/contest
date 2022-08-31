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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut h = &mut head;
        loop {
            match h {
                None => break,
                Some(node) if node.val == val => {
                    *h = node.next.take();
                }
                Some(node) => {
                    h = &mut node.next;
                }
            }
        }
        head
    }
}
