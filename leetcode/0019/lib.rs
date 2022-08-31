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

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = Self::get_length(&head);
        let index = len - n;
        Self::delete_at_index(head, index)
    }

    fn delete_at_index(head: Option<Box<ListNode>>, index: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: 0,
            next: head.clone(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy;
        while idx < index {
            if let Some(ref mut n) = cur.next {
                cur = n;
            }
            idx += 1;
        }
        cur.next = cur.next.take().and_then(|a| a.next);
        return dummy.next;
    }

    fn get_length(head: &Option<Box<ListNode>>) -> i32 {
        if head.is_none() {
            return 0;
        }
        let mut len = 1;
        let mut prev = head.as_ref().unwrap();
        while prev.next.is_some() {
            len += 1;
            prev = prev.next.as_ref().unwrap();
        }
        len
    }
}
