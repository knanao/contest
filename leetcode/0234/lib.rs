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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let n = Self::get_length(&head);
        let mut palindrome = vec![];
        let mut h = &head;
        let mut i = 0;
        while let Some(ref v) = h {
            if i < n / 2 {
                palindrome.push(v.val);
            } else if i != n / 2 || n % 2 == 0 {
                if palindrome.pop().unwrap() != v.val {
                    return false;
                }
            }
            h = &v.next;
            i += 1;
        }
        true
    }

    fn get_length(head: &Option<Box<ListNode>>) -> i32 {
        let mut h = head;
        let mut c = 1;
        while let Some(ref v) = h {
            h = &v.next;
            c += 1;
        }
        c
    }
}
