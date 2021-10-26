use std::usize;

// this function will perform merge sort
pub fn sort(data: &mut [i32]) {
    merge_sort(data, 0, data.len() - 1);
}

fn merge_sort(data: &mut [i32], initial: usize, end: usize) {
    if initial == end {
        return;
    }
    let mid = initial + (end - initial) / 2;
    merge_sort(data, initial, mid);
    merge_sort(data, mid + 1, end);
    merge(data, initial, mid, end);
}

fn merge(data: &mut [i32], initial: usize, mid: usize, end: usize) {
    let mut leftArr = Vec::new();
    let mut rightArr = Vec::new();

    // populate left array

    for index in initial..(mid + 1) {
        leftArr.push(data[index]);
    }

    // populate right array

    for element in (mid + 1)..(end + 1) {
        rightArr.push(data[element]);
    }

    // comparing left and right array and merging

    let (mut x, mut y) = (0usize, 0usize);

    for index in initial..(end + 1) {
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
    pub fn test_sort() {
        let mut list = vec![11, 100, 1, 6, 201, 31];
        sort(list.as_mut_slice());
        assert_eq!(vec![1, 6, 11, 31, 100, 201], list);
    }
}
