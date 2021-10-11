// Creating a immutable linked list .
use std::rc::Rc;

// Basic Strcuture
pub struct Linked_List<T> {
    head: List<T>,
}

//using a ref cell as there can be multiple references for same node at run time
// example   list A -  {2,3}
// list B {0,2,3}
// list c {1,2,3}
// in this case both list b and c are having references of A
type List<T> = Option<Rc<Node<T>>>;
struct Node<T> {
    elem: T,
    next: List<T>,
}

impl<T> Linked_List<T> {
    pub fn new() -> Linked_List<T> {
        Linked_List { head: List::None }
    }

    pub fn prepend(&self, elem: T) -> Linked_List<T> {
        Linked_List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Linked_List<T> {
        Linked_List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// the existing implemenation of drop is bad. Where tail recursion is not possible as the box ref needs
// dropped first .
// the disadantage of this approach is a long stack .
// The below implemnetation acheives custom drop in a optmised way in a while loop
impl<T> Drop for Linked_List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(node) = head {
            // checking first if the RC has only references . then only cleaning . else break
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // testing scenario - node 2 is shared
    // example   list 1 -  {3,2,1}
    // list 2 {2,1}
    // list 5 {5,2,1}
    #[test]
    fn basics() {
        // creating three linked list
        let list = Linked_List::new();
        let list1 = list.prepend(1).prepend(2).prepend(3);
        let list2 = list1.tail();
        let list5 = list2.prepend(5);

        // validating list 1

        assert_eq!(list1.head(), Some(&3));
        let list = list1.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);

        // validating list 2

        assert_eq!(list2.head(), Some(&2));
        let list = list2.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);

        // validating list 5

        assert_eq!(list5.head(), Some(&5));
        let list = list5.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
