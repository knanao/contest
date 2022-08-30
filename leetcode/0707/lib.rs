struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

#[derive(Default)]
struct MyLinkedList {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
impl MyLinkedList {
    fn new() -> Self {
        Default::default()
    }

    fn get(&self, index: i32) -> i32 {
        let mut cur = match self.head {
            Some(ref a) => a,
            None => return -1,
        };
        let mut idx_cur = 0;
        while idx_cur < index {
            if let Some(ref next) = cur.next {
                cur = next;
                idx_cur += 1;
            } else {
                return -1;
            };
        }
        cur.val
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(ListNode {
            val,
            next: self.head.take(),
        }))
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut cur = match self.head {
            Some(ref mut a) => a,
            None => {
                self.head = Some(Box::new(ListNode { val, next: None }));
                return;
            }
        };
        while let Some(ref mut next) = cur.next {
            cur = next;
        }
        cur.next = Some(Box::new(ListNode { val, next: None }));
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut dummy_head = Box::new(ListNode {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            } else {
                return;
            }
            idx += 1;
        }
        cur.next = Some(Box::new(ListNode {
            val,
            next: cur.next.take(),
        }));
        self.head = dummy_head.next;
    }

    fn delete_at_index(&mut self, index: i32) {
        let mut dummy_head = Box::new(ListNode {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = cur.next.take().and_then(|a| a.next);
        self.head = dummy_head.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_linked_list() {
        let obj = MyLinkedList::new();
        let ret_1 = obj.get(1);
        assert_eq!(-1, ret_1);
    }
}
