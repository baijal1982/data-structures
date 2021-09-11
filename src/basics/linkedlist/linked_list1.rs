
pub struct Linked_List<T> {
    head:List<T>
}

type List<T>  = Option<Box<Node<T>>>; 
struct Node<T>  {

    elem:T,
    next: List<T>
}

 
impl<T> Linked_List<T>  {

    pub fn new() -> Linked_List<T>  {
        Linked_List {
            head: List::None
        }
    }

    pub fn push(&mut self, elem:T)   {
          
         let new_node = Node {
             elem,
             next:self.head.take()
         };

         self.head = List::Some(Box::new(new_node));

          }

    pub fn pop(&mut self)  -> Option<T> {
            
           
               self.head.take().map(|node|  {

                self.head = node.next;
                node.elem
              })
            
    }
               
    }

    // the existing implemenation of drop is bad. Where tail recursion is not possible as the box ref needs
    // dropped first .
    // the disadantage of this approach is a long stack . 
    // The below implemnetation acheives custom drop in a optmised way in a while loop 
    impl<T> Drop for Linked_List<T>  {
        fn drop(&mut self) {
           let mut curr_node = self.head.take();

           while let List::Some(mut node) = curr_node   {
               curr_node =  std::mem::replace(&mut node.next, List::None);
           }
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

    #[test] 
pub fn test_linked_Strings() {

     let mut list =  Linked_List ::new();
     let str1= "amit";
     let str2= "akash";
     list.push(str1);
     list.push(str2);

     assert_eq!(Some(str2), list.pop());
     assert_eq!(Some(str1),list.pop());
     assert_eq!(None,list.pop());
    
    }


}


