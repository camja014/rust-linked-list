pub mod list {

    use std::cell::{Ref, RefCell, RefMut};
    use std::option::Option;
    use std::rc::Rc;

    type NodeRef<'a> = Rc<RefCell<ListNode<'a>>>;

    struct ListNode<'a> {
        prev: Option<NodeRef<'a>>,
        next: Option<NodeRef<'a>>,
        val: u32,
    }

    pub struct List<'a> {
        head: Option<NodeRef<'a>>,
        tail: Option<NodeRef<'a>>,
        size: usize,
    }

    impl<'a> ListNode<'a> {
        fn new(val: u32) -> ListNode<'a> {
            return ListNode {
                prev: None,
                next: None,
                val: val,
            };
        }
    }

    impl<'a> List<'a> {
        pub fn new() -> List<'a> {
            return List {
                head: None,
                tail: None,
                size: 0,
            };
        }

        pub fn push_back(&mut self, val: u32) {
            let node: NodeRef = Rc::new(RefCell::new(ListNode::new(val)));
            if self.tail.is_none() {
                // No elements in list
                self.head = Some(node.clone());
            } else {
                // Append new value to end of list.
                let mut tail: RefMut<ListNode> = self.tail.as_mut().unwrap().borrow_mut();
                tail.next = Some(node.clone());
            }
            self.tail = Some(node);
            self.size += 1;
        }

        pub fn push_front(&mut self, val: u32) {
            let node: NodeRef = Rc::new(RefCell::new(ListNode::new(val)));
            if self.head.is_none() {
                self.tail = Some(node.clone());
            } else {
                let mut head = self.tail.as_mut().unwrap().borrow_mut();
                head.prev = Some(node.clone());
            }
            self.head = Some(node);
            self.size += 1;
        }

        pub fn peek_front(&self) -> Option<Ref<u32>> {
            return match self.head.as_ref() {
                None => None,
                Some(v) => Some(Ref::map(v.borrow(), |node| &node.val)),
            };
        }

        pub fn peek_back(&self) -> Option<Ref<u32>> {
            return match self.tail.as_ref() {
                None => None,
                Some(v) => Some(Ref::map(v.borrow(), |node| &node.val)),
            };
        }

        pub fn pop_back(&mut self) {
            let new_tail = match &self.tail {
                None => None,
                Some(t) => match &t.as_ref().borrow().prev {
                    None => None,
                    Some(t) => Some(t.clone()),
                },
            };
            self.tail = new_tail;
        }

        pub fn pop_front(&mut self) {
            let new_head = match &self.head {
                None => None,
                Some(t) => match &t.as_ref().borrow().next {
                    None => None,
                    Some(t) => Some(t.clone()),
                },
            };
            self.head = new_head;
        }

        pub fn size(&self) -> usize {
            return self.size;
        }
    }

    /// 'a: ListNode lifetime
    /// 'b: Returned Ref lifetime
    ///
    /// Assert that 'a must be greater than 'b (i.e the list must outlive the iterator)
    struct ListIterator<'a> {
        current: Option<NodeRef<'a>>,
    }

    impl<'a> Iterator for ListIterator<'a> {
        type Item = Ref<'a, u32>;

        fn next(&mut self) -> Option<Self::Item> {
            let next = match self.current.as_ref() {
                None => None,
                Some(v) => v.borrow().next,
            };
            self.current = next;

            let cur_ref: Option<&'a NodeRef> = self.current.as_ref();
            return cur_ref.map(|node| Ref::map(node.borrow(), |node_ref| &node_ref.val));
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn assert_front_is(list: &List, val: u32) {
            let front = list.peek_front();
            assert!(front.is_some());
            assert_eq!(val, *front.unwrap());
        }

        fn assert_back_is(list: &List, val: u32) {
            let back = list.peek_back();
            assert!(back.is_some());
            assert_eq!(val, *back.unwrap());
        }

        #[test]
        fn push_front_single() {
            let mut list = List::new();
            list.push_front(1);
            assert_eq!(1, list.size());
            assert_front_is(&list, 1);
        }

        #[test]
        fn push_front_multi() {
            let mut list = List::new();
            list.push_front(1);
            list.push_front(2);
            list.push_front(3);
            assert_eq!(3, list.size());
            assert_front_is(&list, 3);
        }

        #[test]
        fn push_back_single() {
            let mut list = List::new();
            list.push_back(1);
            assert_eq!(1, list.size());
            assert_back_is(&list, 1);
        }

        #[test]
        fn push_back_multi() {
            let mut list = List::new();
            list.push_back(1);
            list.push_back(2);
            list.push_back(3);
            assert_eq!(3, list.size());
            assert_back_is(&list, 3);
        }

        #[test]
        fn peek_then_pop() {
            let mut list = List::new();
            list.push_back(1);
            let val = list.peek_front().map(|val_ref| *val_ref);
            list.pop_back();
            assert!(val.is_some());
            assert_eq!(1, val.unwrap());
        }
    }
}
