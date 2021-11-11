// basic implementation of linked list

pub struct Linked_List {
    head: List,
}

pub enum List {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    elem: u32,
    next: List,
}

impl Linked_List {
    pub fn new() -> Linked_List {
        Linked_List { head: List::Empty }
    }

    pub fn push(&mut self, elem: u32) {
        let new_node = Node {
            elem,
            next: std::mem::replace(&mut self.head, List::Empty),
        };

        self.head = List::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<u32> {
        let result;
        match std::mem::replace(&mut self.head, List::Empty) {
            List::Empty => {
                result = None;
            }
            List::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }
        result
    }
}

// the existing implemenation of drop is bad. Where tail recursion is not possible as the box ref needs
// dropped first .
// the disadantage of this approach is a long stack .
// The below implemnetation acheives custom drop in a optmised way in a while loop
impl Drop for Linked_List {
    fn drop(&mut self) {
        let mut curr_node = std::mem::replace(&mut self.head, List::Empty);

        while let List::More(mut node) = curr_node {
            curr_node = std::mem::replace(&mut node.next, List::Empty);
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

        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }
}
