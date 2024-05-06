fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged_arr = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged_arr.push(arr1[i]);
            i += 1;
        } else {
            merged_arr.push(arr2[j]);
            j += 1;
        }
    }

    merged_arr.extend_from_slice(&arr1[i..]);
    merged_arr.extend_from_slice(&arr2[j..]);

    merged_arr
}

fn main() {
    let arr1 = [1, 3, 5, 7, 9];
    let arr2 = [2, 4, 6, 8, 10];

    let merged_arr = merge_sorted_arrays(&arr1, &arr2);

    println!("Merged Sorted Array: {:?}", merged_arr);
}