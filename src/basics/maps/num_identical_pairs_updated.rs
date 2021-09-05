use std::collections::HashMap;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
 
      nums.iter().fold(HashMap::new(), |mut acc, k| {
            *acc.entry(k).or_insert(0) += 1;
            acc
        }).values().fold(0, |acc, n| acc + n * (n-1) / 2)
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