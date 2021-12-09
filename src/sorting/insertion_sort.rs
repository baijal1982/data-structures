//insertion sort in ascending order
pub fn sort( data: &mut Vec<i32>)  {
    for j in 1..data.len() {
        let key = data[j];
        let mut i = j;

        while i > 0 && data[i - 1] > key {
            data[i] = data[i - 1];
            i -= 1;
        }
        data[i] = key;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

   

    #[test]
    fn test_sort()    {
        let mut data= vec![2,8,7,1,3,5,6,4];
       sort(&mut data);
       
    assert_eq!(data,vec![1,2,3,4,5,6,7,8]);
    }
}
