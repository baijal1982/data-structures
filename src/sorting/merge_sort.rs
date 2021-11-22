use std::usize;

// this function will perform merge sort
pub fn sort(data: &mut [i32]) {
    merge_sort(data);
}

fn merge_sort(data: &mut [i32]) {
    let mid = data.len() / 2;

    if mid == 0 {
        return;
    }

    merge_sort(&mut data[..mid]);
    merge_sort(&mut data[mid..]);
    let mut ret = data.to_vec();
    merge(&mut ret[..], &data[..mid], &data[mid..]);
    data.copy_from_slice(&ret);
}

fn merge(data: &mut [i32], leftArr: &[i32], rightArr: &[i32]) {
    // comparing left and right array and merging

    let (mut x, mut y) = (0usize, 0usize);

    for index in 0..data.len() {
        if x < leftArr.len() && y < rightArr.len() && leftArr[x] < rightArr[y] {
            data[index] = leftArr[x];
            x += 1;
        } else if y < rightArr.len() {
            data[index] = rightArr[y];
            y += 1;
        } else if x < leftArr.len() {
            data[index] = leftArr[x];
            x += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_sort_basic() {
        let mut list = vec![4, 3, 2, 1];
        sort(list.as_mut_slice());
        assert_eq!(vec![1, 2, 3, 4], list);
    }

    #[test]
    pub fn test_sort_large() {
        let mut list = vec![11, 100, 1, 6, 201, 31];
        sort(list.as_mut_slice());
        assert_eq!(vec![1, 6, 11, 31, 100, 201], list);
    }
}
