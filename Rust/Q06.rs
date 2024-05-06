fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    let first_str = strs[0].as_str();
    
    strs.iter().skip(1).fold(first_str.to_string(), |prefix, s| {
        prefix.chars().zip(s.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect()
    })
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    
    let common_prefix = longest_common_prefix(strings);
    
    println!("The longest common prefix is: {}", common_prefix);
}