fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let reversed_input = input.chars().rev().collect::<String>();
    input == reversed_input
}

fn main() {
    let input = "A man, a plan, a canal, Panama";
    if is_palindrome(input) {
        println!("The input string is a palindrome.");
    } else {
        println!("The input string is not a palindrome.");
    }
}