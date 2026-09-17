pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = (low + high) / 2;

        if key == array[mid] {
            return Some(mid);
        } else if key < array[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    None
}
