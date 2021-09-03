// this method will find a peek in the input array using binary search
pub fn find_peek(data: Vec<u32>, initial_value: usize, final_value: usize, length: usize) {
    let mid = initial_value + (final_value - initial_value) / 2;

    // best case if mid value  is peek
    if (initial_value == 0 || data[mid] > data[mid - 1])
        && (final_value == length || data[mid] > data[mid + 1])
    {
        println!("the peek value {} exist at index {}", data[mid], mid);
        return;
    }
    // searching towards right
    else if mid < data.len() - 1 && data[mid + 1] > data[mid] {
        find_peek(data, mid + 1, final_value, length);
    }
    // searching towards left
    else {
        find_peek(data, initial_value, mid, length);
    }
}
