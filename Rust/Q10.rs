fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    
    let mut is_prime = true;
    
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            is_prime = false;
            break;
        }
    }
    
    is_prime
}

fn main() {
    let num = 17;
    
    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}