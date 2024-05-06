fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        let mid1 = arr[(n / 2) - 1];
        let mid2 = arr[n / 2];
        return (mid1 + mid2) as f64 / 2.0;
    } else {
        return arr[n / 2] as f64;
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let median = find_median(&arr);
    println!("Median of the array is: {}", median);
}