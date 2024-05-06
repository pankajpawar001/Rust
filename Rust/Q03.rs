fn shortest_word_in_string(input: &str) -> &str {
    input.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn main() {
    let input = "This is a test string with words of varying lengths";
    let shortest_word = shortest_word_in_string(input);
    println!("The shortest word in the string is: {}", shortest_word);
}