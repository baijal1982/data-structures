
// This function will reverse a integer .
pub fn reverse(x : i32 )  ->  i32 {

    let mut num =x ;

    if !(x <= i32::MAX  || x>= i32::MIN)   {
        return   0;
    }   
    let mut reverse:i32= 0 ;
    while num!= 0 {
   
          let last_digit = num%10;
          let temp_num1   = reverse.checked_mul(10);

          match temp_num1   {

            Some(temp_num2) => {
                  let temp_num3  = temp_num2.checked_add(last_digit);
                  match temp_num3   {
                      None =>   {
                          return   0;
                      },
                      Some(temp_num4)  =>   {
                          reverse  = temp_num4;
                      }

                  }
            },
            None =>  {
                return 0;
            }
            
          } 

          num = num/10;
          
    }

    reverse
}

#[cfg(test)]  
mod tests {

use super::*;

#[test]
pub fn test_reverse()  {

      let num = 1234;
      assert_eq!(4321,reverse(num));

}


#[test]
pub fn test_negative_num()  {

    let num = -1234;
    assert_eq!(-4321,reverse(num));

}

#[test]
pub fn test_reverse_overflow()  {

    let num = -1234;
    assert_eq!(0,reverse(i32::MAX));

}
}

