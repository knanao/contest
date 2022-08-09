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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut to_remove = head.as_ref().unwrap().val - 1;
        let mut dummy = Some(Box::new(ListNode {
            next: head,
            val: to_remove,
        }));
        let mut node = &mut dummy.as_mut().unwrap().next;
        loop {
            match node {
                None => return dummy.unwrap().next,
                Some(n) if n.val == to_remove => *node = n.next.take(),
                Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => {
                    to_remove = n.val
                }
                Some(n) => {
                    node = &mut n.next;
                    if let Some(n) = node {
                        to_remove = n.val - 1
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        // [1,2,3,3,4,4,5]
        let input = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode {
                            next: Some(Box::new(ListNode {
                                next: Some(Box::new(ListNode { next: None, val: 5 })),
                                val: 4,
                            })),
                            val: 4,
                        })),
                        val: 3,
                    })),
                    val: 3,
                })),
                val: 2,
            })),
            val: 1,
        }));
        let got = Solution::delete_duplicates(input);
        let want = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 5 })),
                val: 2,
            })),
            val: 1,
        }));
        assert_eq!(got, want);
    }
}
