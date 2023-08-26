pub fn swap<T>(first: &mut T, second: &mut T) {
    unsafe {
        std::ptr::swap(first, second);
    }
}

pub fn find_min_index<T: Ord>(array: &[T], start_idx: usize) -> usize {
    let mut min_idx = start_idx;
    for i in (start_idx + 1)..array.len() {
        if array[i] < array[min_idx] {
            min_idx = i;
        }
    }
    min_idx
}

pub fn selection_sort<T: Ord>(array: &mut [T]) -> () {
    for i in 0..array.len() {
        let min_idx = find_min_index(array, i);
        if min_idx != i {
            array.swap(min_idx, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![93, 19, -23, -33, 294, 0, 123, 0];
        let mut actual = arr.clone();

        selection_sort(&mut actual);
        arr.sort();

        assert_eq!(arr, actual);
    }

    #[test]
    fn test_swap() {
        let mut a = 10;
        let mut b = 20;
        swap(&mut a, &mut b);
        assert_eq!(a, 20);
        assert_eq!(b, 10);
    }
}
