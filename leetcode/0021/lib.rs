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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1 == None {
            return list2;
        }
        if list2 == None {
            return list1;
        }

        let mut node1 = list1.unwrap();
        let mut node2 = list2.unwrap();
        if node1.val < node2.val {
            node1.next = Self::merge_two_lists(node1.next, Some(node2));
            return Some(node1);
        }
        if node1.val >= node2.val {
            node2.next = Self::merge_two_lists(Some(node1), node2.next);
            return Some(node2);
        }
        None
    }
}
