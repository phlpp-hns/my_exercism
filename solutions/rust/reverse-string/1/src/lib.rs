pub fn reverse(input: &str) -> String {
    let output = input.to_string();
    output.chars().rev().collect()
}
