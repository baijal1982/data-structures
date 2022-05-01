use std::vec;

 struct  Union_Find_W {

    nodes: Vec<u32>,
    components: u32,
    weights: Vec<u32>,
    size: u32

}



trait Union_Find  {

    fn is_connected(&mut self,p:u32,q:u32) -> bool ;
    fn union(&mut self,p:u32,q:u32);
    fn components(&mut self) -> u32;

}

impl Union_Find_W  {
    pub fn new(n:u32)-> Union_Find_W  {
        let mut nodes_temp = Vec::<u32>::new();
        let mut weights_temp = Vec::<u32>::new();
        for  i in 0..n  {
            nodes_temp.push(i);
            weights_temp.push(1);
        }

            Union_Find_W  {
         size: n,
         components : n,
         nodes: nodes_temp,
         weights : weights_temp,
            
            }
            
    }
    fn validate (&mut self , p:u32 ,q : u32 )  {
        if p >= self.size || q >= self.size   {
            panic!("index out of bounds");
        }
    }

  

    fn get_root(&mut self, p: u32) -> usize {
          self.validate(p,0);
          let mut index  = p ;
           while self.nodes[index as usize] != index as u32 {
               index = self.nodes[index as usize];
           }
           index as usize
    }
}

impl Union_Find for Union_Find_W   {
    fn is_connected(&mut self,p:u32,q:u32) -> bool  {
       self.validate(p, q);
        self.nodes[p as usize] == self.nodes[q as usize]
    }

   

    fn union(&mut self,p:u32,q:u32) {
        self.validate(p, q);
        let root_p = self.get_root(p);
        let root_q=  self.get_root(q);

        if root_p  == root_q   {
            return;
        }


        if self.weights[root_p] > self.weights[root_q]   {
            self.weights[root_p] =self.weights[root_p] +self.weights[root_q]  ;
            self.nodes[root_q] = root_p as u32;

        }
        else if self.weights[root_p] < self.weights[root_q] {
            self.weights[root_q] =self.weights[root_q] +self.weights[root_p]  ;
            self.nodes[root_p] = root_q as u32;


        } 
        else {
            self.weights[root_p] =self.weights[root_p] +self.weights[root_q]  ;
            self.nodes[root_q] = root_p as u32;
        }
        self.components= self.components-1;
        
    }

    fn components(&mut self) -> u32 {
        self.components
    }
}

#[cfg(test)]
mod test {
    use super::*;


   

    #[test]
    fn test_function_full() {

      let mut data = Union_Find_W::new(10);
      data.union(4, 3);
      data.union(3, 8);
      data.union(6, 5);
      data.union(9, 4);
    data.union(2, 1);
     data.union(5, 0);
     data.union(7, 2);
    data.union(6, 1);
     data.union(7, 3);

      let current_data = vec![6,2,6,4,6,6,6,2,4,4];
      assert_eq!(data.nodes,current_data);

    }
}