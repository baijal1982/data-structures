use std::ops::{Deref, DerefMut};

// this implementation provids implementation for Stack using linked list

pub struct Linked_List<T> {
    head: List<T>,
}

type List<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: List<T>,
}

struct Iter<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

pub struct Into_Iter<T>(Linked_List<T>);

impl<T> Linked_List<T> {
    pub fn new() -> Linked_List<T> {
        Linked_List { head: List::None }
    }

    pub fn into_iter(self) -> Into_Iter<T> {
        Into_Iter(self)
    }

    pub fn iter<'a>(&'a mut self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_mut().map(|node| node.deref_mut()),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };

        self.head = List::Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Iterator for Into_Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// the existing implemenation of drop is bad. Where tail recursion is not possible as the box ref needs
// dropped first .
// the disadantage of this approach is a long stack .
// The below implemnetation acheives custom drop in a optmised way in a while loop
impl<T> Drop for Linked_List<T> {
    fn drop(&mut self) {
        let mut curr_node = self.head.take();

        while let List::Some(mut node) = curr_node {
            curr_node = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_linked_list() {
        let mut list = Linked_List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(Some(4), list.pop());
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(&2), list.peek());
        assert_eq!(Some(2), list.pop());

        let mut num = list.peek_mut();
        let num1 = num.expect("taking value");

        *num1 = 10;
        assert_eq!(Some(10), list.pop());
    }

    #[test]
    pub fn test_linked_Strings() {
        let mut list = Linked_List::new();
        let str1 = "amit";
        let str2 = "akash";
        list.push(str1);
        list.push(str2);

        assert_eq!(Some(str2), list.pop());
        assert_eq!(Some(str1), list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    pub fn test_into_iter() {
        let mut list = Linked_List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    pub fn test_iter() {
        let mut list = Linked_List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        assert_eq!(Some(&mut 3), iter.next());
        assert_eq!(Some(&mut 2), iter.next());
        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    pub fn test_mut_iter() {
        let mut list = Linked_List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let data = list.iter().next();
        assert_eq!(Some(&mut 3), data);
        data.map(|x| *x += 1);

        let data1 = list.iter().next();

        assert_eq!(Some(&mut 4), data1);
    }
}
