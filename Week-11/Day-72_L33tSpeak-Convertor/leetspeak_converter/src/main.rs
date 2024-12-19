use std::collections::HashMap;

fn to_leetspeak(input: &str, mappings: &HashMap<char, char>) -> String {
    let mut result = String::new();

    for c in input.to_uppercase().chars() {
        match c {
            'W' => result.push_str("//"),
            'M' => result.push_str("(V)"),
            _ => match mappings.get(&c) {
                Some(&mapped_char) => result.push(mapped_char),
                None => result.push(c),
            },
        }
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

    let input = "storm";
    let leetspeak = to_leetspeak(input, &leetspeak_mappings);
    println!("{} => {}", input, leetspeak);
}
