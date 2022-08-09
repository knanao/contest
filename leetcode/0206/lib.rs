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

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h = head;
        let mut p = h.as_mut();
        let mut stack: Vec<i32> = vec![];
        while let Some(node) = p {
            stack.push(node.val);
            p = node.next.as_mut();
        }

        let mut ret = Some(Box::new(ListNode::new(0)));
        let mut n = ret.as_mut().unwrap();
        while !stack.is_empty() {
            let v = stack.pop().unwrap();
            n.next = Some(Box::new(ListNode::new(v)));
            n = n.next.as_mut().unwrap();
        }
        ret.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
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
        let got = Solution::reverse_list(input);
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
