use std::cmp::max;
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        
    if nums.is_empty() {
        return   0;
    }

    return  max_sum(&nums, 0,nums.len()-1);
}


fn max_sum(nums:&Vec<i32>,initial:usize ,last:usize)  -> i32 {



    if initial == last   {
        return   nums[initial];
    }

    let mid = initial  + (last - initial)/2;

    let max_left = max_sum(nums, initial, mid);
    let max_right=  max_sum(nums, mid+1, last);
    let max_mid = max_mid_array(nums, initial, last, mid);
    max(max(max_left,max_right), max_mid)
}


fn max_mid_array(nums:&Vec<i32>,initial:usize ,last:usize, mid:usize)  -> i32 {

    let ( mut result_left, mut result) =(i32::MIN,0);

    for i in (initial..=mid).rev()  {
        result = result + nums[i];

        if result > result_left  {
            result_left = result;
        }
    } 


    let ( mut result_right, mut result) =(i32::MIN,0);

    for i in mid+1..=last  {
        result = result + nums[i];

        if result > result_right  {
            result_right = result;
        }
    } 


    max(max(result_left,result_right), result_left+result_right)

}


#[cfg(test)]

mod test {

use super::*;
    #[test] 
    pub fn max_sub_array_success1()  {
        let nums = vec!(-6,-2,8,3,4,-2) ;
        assert_eq!(15,max_sub_array(nums));
    }

    #[test] 
    pub fn max_sub_array_success2()  {
        let nums = vec!(-3,2,5,6,7,1,-3,-2) ;
        assert_eq!(21,max_sub_array(nums));
    }

    #[test]
    pub fn max_sub_array_empty()  {
        let nums = vec!() ;
        assert_eq!(0,max_sub_array(nums));
    }

    #[test]
    pub fn max_sub_unit_array()  {
        let nums = vec!(1) ;
        assert_eq!(1,max_sub_array(nums));
    }
}

