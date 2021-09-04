use std::collections::HashMap;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
 

    if nums.len() < 2   {
        return 0;
    }
    
    let mut map:HashMap<i32,Vec<usize>> = HashMap::new();
     let mut  count:i32 =0;
    for index in 0..nums.len()  {
         if map.contains_key(&nums[index])  {
               let mut list =  map.get(&nums[index]).expect("").to_vec();
               list.push(index);
              map.insert(nums[index], list);
              count = count + map.get(&nums[index]).expect("").len() as i32 -1;
         }
         else {
             map.insert(nums[index],vec![index]);
         }

    }
    count
}

#[cfg(test)]
mod tests {


    use crate::basics::maps::num_identical_pairs;

    use super::*;

    #[test]
    fn test_happy_path()  {

          assert_eq!(num_identical_pairs(vec![1,2,3,1,1,3]),4);

    }

    #[test]
    fn test_complex_case()  {

          assert_eq!(num_identical_pairs(vec![1,1,1,1]),6);

    }

    #[test]
    fn test_unique_case()  {

          assert_eq!(num_identical_pairs(vec![1,2,3]),0);

    }

    #[test]
    fn test_single_element()  {

          assert_eq!(num_identical_pairs(vec![1]),0);

    }

    #[test]
    fn test_empty()  {

          assert_eq!(num_identical_pairs(vec![]),0);

    }

}