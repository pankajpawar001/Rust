fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=(number as f64).sqrt() as u32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let number = 17;
    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}