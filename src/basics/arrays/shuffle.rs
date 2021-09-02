
// This program will shuffle array in the below format 
//Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].

//Return the array in the form [x1,y1,x2,y2,...,xn,yn].
// This runs in time complexity O(n/2) and spsace complexity O(n)
pub fn shuffle(array:Vec<i32>,n:u32) -> Vec<i32>  {
    
    if n <3 {
        return  array.to_vec();
    }
    let mut new_array = vec![0;n as usize];

    if n%2 !=0 {
        new_array[(n-1) as usize] = array[(n-1) as usize];
    }

    let mid= n/2;
    for i in 0..mid {
        new_array[(i*2) as usize] = array[i as usize];
          let other_mid_index = mid +i;
          new_array[(other_mid_index-(mid-(i+1))) as usize] = array[other_mid_index as usize];
    }

    new_array
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_even_elements() {
        let  data= vec![2,5,1,3,4,7];
        let expected= vec![2,3,5,4,1,7];
        assert_eq!( expected, shuffle(data,6));
    }
    #[test]
    fn test_odd_elements() {
        let  data= vec![2,5,1,3,4];
        let expected= vec![2,1,5,3,4];
        assert_eq!( expected, shuffle(data,5));
    }
    #[test]
    fn test_single_element() {
        let  data= vec![10];
        let expected= vec![10];
        assert_eq!( expected, shuffle(data,1));
    }

    #[test]
    fn test_negative_elements() {
        let  data= vec![-2,5,-1,3,4,-7];
        let expected= vec![-2,3,5,4,-1,-7];
        assert_eq!( expected, shuffle(data,6));
    }



}