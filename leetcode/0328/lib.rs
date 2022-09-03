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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut left = ListNode::new(-1);
        let mut l = &mut left;
        let mut right = ListNode::new(-1);
        let mut r = &mut right;
        let mut i = 1;
        while let Some(mut n) = head {
            head = n.next.take();
            if i % 2 == 0 {
                l.next = Some(n);
                l = l.next.as_mut().unwrap();
            } else {
                r.next = Some(n);
                r = r.next.as_mut().unwrap();
            }
            i += 1;
        }
        r.next = left.next;
        right.next
    }
}
