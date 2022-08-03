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

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.clone();
        let mut l2 = l2.clone();
        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;

        let mut carry = 0;
        let mut v1 = 0;
        let mut v2 = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            if let Some(node) = l1 {
                v1 = node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                v2 = node.val;
                l2 = node.next;
            }

            current.next = Some(Box::new(ListNode::new((v1 + v2 + carry) % 10)));
            current = current.next.as_mut().unwrap();
            carry = (v1 + v2 + carry) / 10;
        }

        head.next
    }
}
