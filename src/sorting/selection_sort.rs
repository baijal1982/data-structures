
// selection sort 
pub fn sort ( arr: &mut[i32] , n: usize)  {
    // base case 
    if n ==1  {
        return;
    }

       let max_index = find_largest_index(arr, n) ;
       arr.swap(n-1, max_index) ;
       sort(arr, n-1);
}


fn find_largest_index( arr:&mut[i32] , n:usize)  -> usize    {
    let (mut max_index, mut max_num)=(0,0);
    for i in 0..n {
           if arr[i]  > max_num   {
               max_num  =arr[i];
               max_index =i;
           }
    }
    max_index
}

#[cfg(test)]
mod tests {

    use super::*;

   

    #[test]
    fn test_sort()    {
        let mut data= vec![2,8,7,1,3,5,6,4];
       sort(&mut data,8);
       
    assert_eq!(data,vec![1,2,3,4,5,6,7,8]);
    }
}