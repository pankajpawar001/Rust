fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let sorted_arr = [1, 3, 5, 5, 7, 9, 11];
    let target_num = 5;

    match find_first_occurrence(&sorted_arr, target_num) {
        Some(index) => println!("The first occurrence of {} is at index {}.", target_num, index),
        None => println!("The number {} is not found in the array.", target_num),
    }
}