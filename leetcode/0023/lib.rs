use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

struct Solution;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for mut node in lists {
            if node.is_some() {
                heap.push(node.take()?);
            }
        }

        let mut head = heap.pop()?;
        let mut p = &mut head;
        while !heap.is_empty() {
            if p.next.is_some() {
                heap.push(p.next.take()?);
            }

            p.next = Some(heap.pop()?);
            p = p.next.as_mut()?;
        }
        Some(head)
    }
}
