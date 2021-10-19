use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

// Basic Strcuture
pub struct Linked_List<T> {
    head: List<T>, // ref to the head node
    tail: List<T>, // to the last node
}

//using a ref cell as there can be multiple references for same node at compile time
// example   list A -  {2,3}
// list B {0,2,3}
// list c {1,2,3}
// in this case both list b and c are having references of A
// using a refcell as we want to haveability to manipulate node as well as have it references in pre and next

type List<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    elem: T,
    next: List<T>,
    prev: List<T>,
}

impl<T> Linked_List<T> {
    pub fn new() -> Linked_List<T> {
        Linked_List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }
}

impl<T> Drop for Linked_List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn basics() {
        let mut list = Linked_List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(3));

        assert_eq!(list.pop_front(), Some(2));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
