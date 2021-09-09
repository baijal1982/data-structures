
pub struct Linked_List {
    head:List
}

pub enum List {
    Empty,
    More(Box<Node>)
}

struct Node  {

    elem:u32,
    next: List

}

 


impl Linked_List  {

    pub fn new() -> Linked_List  {
        Linked_List {
            head: List::Empty
        }
    }

    pub fn push(&mut self, elem:u32)   {
          
         let new_node = Node {
             elem,
             next:std::mem::replace(&mut self.head,List::Empty)
         };

         self.head = List::More(Box::new(new_node));

          }

          pub fn pop(&mut self)  -> Option<u32> {
            
            
                let result ;
              match std::mem::replace(&mut self.head, List::Empty)  {
                List::Empty => {
                    result = None;
                }
                List::More(node) =>  {
                   result = Some(node.elem);
                   self.head = node.next;
                    
                }


            }
           result

          }
    }

#[cfg(test)]
mod tests  {

use super::*;


#[test] 
pub fn test_linked_list() {

     let mut list =  Linked_List ::new();
     list.push(1);
     list.push(2);

     assert_eq!(Some(2), list.pop());
     assert_eq!(Some(1),list.pop());
     assert_eq!(None,list.pop());
    
    }

}
