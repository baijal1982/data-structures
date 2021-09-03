//insertion sort in ascending order
pub fn sort(mut data: Vec<i64>) -> Vec<i64> {
    for j in 1..data.len() {
        let key = data[j];
        let mut i = j;

        while i > 0 && data[i - 1] > key {
            data[i] = data[i - 1];
            i -= 1;
        }
        data[i + 1] = key;
    }
    data
}
