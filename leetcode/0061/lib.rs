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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut v: Vec<i32> = Vec::new();
        let mut node = &head;
        while let Some(n) = node {
            v.push(n.val);
            node = &n.next;
        }
        let mut answer = None;
        let j = k as usize % v.len();
        for i in (0..v.len()).rev() {
            answer = Some(Box::new(ListNode {
                val: v[(v.len() + i - j) % v.len()],
                next: answer,
            }))
        }
        answer
    }
}
