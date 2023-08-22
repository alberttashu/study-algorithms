#![allow(unused)]

fn main() {
    p1();
    p2();
}

fn p1() {
    println!("Searching insert position...");
    let vector = vec![1, 3, 5, 9, 15, 17, 34, 66, 323];
    let position = find_insert_position(vector, 1233);
    println!("Finding position: {position}");
}

fn find_insert_position(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let middle_idx = left + (right - left) / 2;
        let middle_value = nums[middle_idx as usize];

        if middle_value == target {
            return middle_idx as i32;
        } else if middle_value < target {
            left = middle_idx + 1;
        } else {
            right = middle_idx - 1;
        }
    }
    return left;
}

fn p2() {
    println!("Calculating sqrt...");
    let x = 1230000;
    let sqrt = sqrt(x);
    println!("Sqrt({x}) = {sqrt}")
}

fn sqrt(value: i32) -> i32 {
    let mut left = 0;
    let mut right = value;

    while left <= (right - 1) {
        let mut candidate = left + (right - left + 1) / 2;
        let x = candidate as i64 * candidate as i64;
        if x == value as i64 {
            return candidate;
        } else if x < value as i64 {
            left = candidate;
        } else {
            right = candidate - 1;
        }
    }
    return left;
}
