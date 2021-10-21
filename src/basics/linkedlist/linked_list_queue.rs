use std::{borrow::Borrow, cell::RefCell, rc::Rc};

// this implementation provids implementation for Queue using linked list

pub struct Linked_List<T> {
    head: List<T>,
    tail: List<T>,
}

type List<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    elem: T,
    next: List<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { elem, next: None }))
    }
}

impl<T> Linked_List<T> {
    pub fn new() -> Linked_List<T> {
        Linked_List {
            head: List::None,
            tail: List::None,
        }
    }

    pub fn enqueue(&mut self, elem: T) {
        let new_item = Node::new(elem);

        match self.tail.take() {
            Some(node) => {
                node.borrow_mut().next = Some(new_item.clone());
                self.tail = Some(new_item);
            }
            None => {
                self.head = Some(new_item.clone());
                self.tail = Some(new_item);
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for Linked_List<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_basics() {
        let mut list = Linked_List::new();
        list.enqueue(1);
        list.enqueue(2);
        list.enqueue(3);
        assert_eq!(list.dequeue(), Some(1));
        assert_eq!(list.dequeue(), Some(2));
        assert_eq!(list.dequeue(), Some(3));
        assert_eq!(list.dequeue(), None);
    }
}
