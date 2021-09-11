//Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same.
// Example
//Input: nums = [1,1,2]
//Output: 2, nums = [1,2,_]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // base case

    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut tail: usize = 1;

    for i in 1..nums.len() {
        if nums[i] != nums[tail] {
            nums[tail] = nums[i];
            tail += 1;
        }
    }

    tail as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_duplicates1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates(&mut nums));
        assert_eq!(vec![1, 2, 2], nums);
    }

    #[test]
    fn test_duplicates2() {
        let mut nums = vec![1, 1, 2, 2, 3];
        assert_eq!(3, remove_duplicates(&mut nums));
        assert_eq!(vec![1, 2, 3, 2, 3], nums);
    }

    #[test]
    fn test_unit_element() {
        let mut nums = vec![1];
        assert_eq!(1, remove_duplicates(&mut nums));
        assert_eq!(vec![1], nums);
    }

    #[test]
    fn test_empty() {
        let mut nums = vec![];
        assert_eq!(0, remove_duplicates(&mut nums));
    }
}
