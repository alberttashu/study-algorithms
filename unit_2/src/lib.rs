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

pub fn insertion_sort<T: Ord>(array: &mut [T]) -> () {
    for sorted_boundary_idx in 0..array.len() {
        let mut j = sorted_boundary_idx;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn factorial_iter(n: i32) -> i32 {
    if n < 0 {
        panic!("Negative integers not supported");
    }
    let mut result = 1;
    for i in 1..n + 1 {
        result *= i;
    }
    result
}

pub fn factorial_rec(n: i32) -> i32 {
    if n < 0 {
        panic!("Negative integers not supported");
    } else if n == 1 || n == 0 {
        return 1;
    }
    return n * factorial_rec(n - 1);
}

pub fn is_palindrome(str: &str) -> bool {
    if str.len() <= 1 {
        return true;
    } else if str.chars().last() != str.chars().next() {
        return false;
    }
    return is_palindrome(&str[1..str.len() - 1]);
}

pub fn recursive_pow(n: f64, p: i32) -> f64 {
    match p {
        0 => 1.0,
        -1 => 1.0 / n,
        _ if p > 0 && p % 2 == 0 => {
            let y = recursive_pow(n, p / 2);
            y * y
        }
        _ if p > 0 => n * recursive_pow(n, p - 1),
        _ => recursive_pow(n, p + 1) / n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![93, 19, -23, -33, 294, 0, 123, 0];
        let mut actual = arr.clone();

        insertion_sort(&mut actual);
        arr.sort();

        assert_eq!(arr, actual);
    }

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

    #[test]
    fn test_factorial_rec() {
        let n = 5;
        let expected = 120;
        let actual = factorial_rec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_factorial_iter() {
        let n = 5;
        let expected = 120;
        let actual = factorial_iter(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_palindrome() {
        let palindrome = "rotor";
        let non_palindrome = "motor";

        assert_eq!(true, is_palindrome(palindrome));
        assert_eq!(false, is_palindrome(non_palindrome));
    }

    #[test]
    fn test_recursive_pow() {
        assert_eq!(recursive_pow(2.0, 5), 32.0);
        assert_eq!(recursive_pow(0.0, 5), 0.0);
        assert_eq!(recursive_pow(2.0, -2), 0.25);
        assert_eq!(recursive_pow(2.0, 1), 2.0);
    }
}
