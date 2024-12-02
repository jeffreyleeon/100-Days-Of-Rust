fn to_hex(s: &str) -> String {
    // Initialize an empty vector to store the hexadecimal values
    let mut hex_values: Vec<String> = Vec::new();

    // Iterate over each character in the string
    for char in s.chars() {
        // Convert the character to its Unicode code point (ASCII value for ASCII characters)
        // and then to a hexadecimal string
        let hex_value = format!("{:02x}", char as u8);
        
        // Append the hexadecimal value to the vector
        hex_values.push(hex_value);
    }

    // Join the hexadecimal values with spaces and return the result
    hex_values.join(" ")
}

fn main() {
    // Example usage:
    println!("{}", to_hex("hello world"));  // Output: "68 65 6c 6c 6f 20 77 6f 72 6c 64"
    println!("{}", to_hex("Big Boi"));      // Output: "42 69 67 20 42 6f 69"
    println!("{}", to_hex("Marty Poppinson"));  // Output: "4d 61 72 74 79 20 50 6f 70 70 69 6e 73 6f 6e"
}