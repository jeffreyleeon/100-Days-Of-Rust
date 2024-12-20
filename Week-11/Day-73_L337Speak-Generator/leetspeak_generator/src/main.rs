use std::collections::HashMap;

fn from_leetspeak(input: &str, mappings: &HashMap<char, char>) -> String {
    let mut result = String::new();
    let mut temp = String::new();

    for c in input.chars() {
        temp.push(c);

        // Handle special cases
        if temp == "//" {
            result.push('W');
            temp.clear();
        } else if temp == "(V)" {
            result.push('M');
            temp.clear();
        } else {
            // Check if the current character is a mapped character
            let reverse_mappings: HashMap<char, char> = mappings.iter().map(|(k, v)| (*v, *k)).collect();
            match reverse_mappings.get(&c) {
                Some(&original_char) => {
                    result.push(original_char);
                    temp.clear();
                }
                None => {
                    // If not a mapped character, just append it
                    if temp.len() == 1 {
                        result.push(c);
                        temp.clear();
                    }
                }
            }
        }
    }

    // Handle any remaining characters in the temp buffer
    if !temp.is_empty() {
        result.push_str(&temp);
    }

    result
}

fn main() {
    let leetspeak_mappings = HashMap::from([
        ('A', '4'),
        ('B', '6'),
        ('E', '3'),
        ('I', '1'),
        ('N', '\\'),
        ('O', '0'),
        ('S', '5'),
        ('T', '7'),
        ('V', '/'),
    ]);

    let leetspeak = "570R(V)";
    let original = from_leetspeak(&leetspeak, &leetspeak_mappings);
    println!("{} => {}", leetspeak, original);
}