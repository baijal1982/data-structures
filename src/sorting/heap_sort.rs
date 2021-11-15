// Heap Sort algo implemnetation in RUST 

pub struct Heap  {
    size: usize,
    data:Vec<u32>
}

impl Heap   {
    pub fn new(arr: &mut [u32] )-> Heap  {


        Heap { size: arr.len(), data: arr.to_vec() }
    }


    pub fn max_heapify( &mut self ,n:usize)    {

        if n*2 +1 < self.size  || n*2 +2 < self.size  {

            let left = n*2 +1  ;
            let right = n*2 +2  ;

            let mut  max_index  = n ;

            let  arr = &self.data;

            if  left<self.size && arr[n]  <= arr[left]   {
                max_index = left ;
            }
            if right<self.size && arr[max_index]  <= arr[right]   {
                max_index = right;
            }

            if max_index !=n   {
               self.swap(n,max_index);
                self.max_heapify(max_index);
            } 
        }   
    }


     fn  swap( &mut self, source:usize , dest : usize)   {
        let temp = self.data[dest];
        self.data[dest] = self.data[source];
        self.data[source] = temp;
    }


    pub  fn build_max_heap(&mut self)   {
        let mid = self.size/2;
        for i in (0..mid).rev()   {
            self.max_heapify(i);
        }
    }


    pub fn sort(&mut self)   {
        self.build_max_heap();
        for i in (1..self.size).rev()   {
            self.swap(i, 0);
            self.size -=1;
            self.max_heapify(0);
        }

        
    }


}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_heapify()    {
        let mut data= vec![5,13,2,25,7,17];
        let mut heap = Heap::new(&mut data);
        heap.max_heapify(1);
    assert_eq!(heap.data,vec![5,25,2,13,7,17]);
    }

    #[test]
    fn test_build_heap()    {
        let mut data= vec![5,13,2,25,7,17];
        let mut heap = Heap::new(&mut data);
        heap.build_max_heap();
    assert_eq!(heap.data,vec![25,13,17,5,7,2]);
    }

    #[test]  
    fn test_sort() {
        let mut data= vec![5,13,2,25,7,17];
        let mut heap = Heap::new(&mut data);
        heap.sort();
    assert_eq!(heap.data,vec![2,5,7,13,17,25]);
    }

}