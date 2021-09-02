
// This program will shuffle array in the below format 
//Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].

//Return the array in the form [x1,y1,x2,y2,...,xn,yn].
// This runs in time complexity O(n/2) and spsace complexity O(n)
pub fn shuffle(array:&mut Vec<i32>,n:u32) -> Vec<i32>  {
    
    if n <3 {
        return  array.to_vec();
    }
    let mut newArray = vec![0;n as usize];

    if n%2 !=0 {
        newArray[(n-1) as usize] = array[(n-1) as usize];
    }

    let mid= n/2;
    for i in 0..mid {
          newArray[(i*2) as usize] = array[i as usize];
          let other_mid_index = mid +i;
          newArray[(other_mid_index-(mid-(i+1))) as usize] = array[other_mid_index as usize];
    }

   newArray
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_even_elements() {
        let mut data= vec![2,5,1,3,4,7];
        let expected= vec![2,3,5,4,1,7];
        assert_eq!( expected, shuffle(&mut data,6));
    }
    #[test]
    fn test_odd_elements() {
        let mut data= vec![2,5,1,3,4];
        let expected= vec![2,1,5,3,4];
        assert_eq!( expected, shuffle(&mut data,5));
    }
    #[test]
    fn test_single_element() {
        let mut data= vec![10];
        let expected= vec![10];
        assert_eq!( expected, shuffle(&mut data,1));
    }

    #[test]
    fn test_negative_elements() {
        let mut data= vec![-2,5,-1,3,4,-7];
        let expected= vec![-2,3,5,4,-1,-7];
        assert_eq!( expected, shuffle(&mut data,6));
    }



}