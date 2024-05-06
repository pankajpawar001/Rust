fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

fn main() {
    let arr = [30, 15, 20, 10, 25];
    let k = 3;

    match kth_smallest(&arr, k) {
        Some(kth_smallest) => println!("The {}th smallest element in the array is {}", k, kth_smallest),
        None => println!("Invalid value of k"),
    }
}