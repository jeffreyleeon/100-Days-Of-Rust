fn encrypt(text: String) -> String {
    let mut chars: Vec<char> = text.chars().rev().collect();
    for ch in &mut chars {
        *ch = match *ch {
            'a' => '0',
            'e' => '1',
            'i' => '2',
            'o' => '2',
            'u' => '3',
            _ => *ch,
        };
    }
    let mut cipher_text: String = chars.into_iter().collect();
    cipher_text.push_str("aca");
    cipher_text
}

fn main() {
    let text = String::from("apple");
    let result = encrypt(text);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt(String::from("apple")), "1lpp0aca");
        assert_eq!(encrypt(String::from("banana")), "0n0n0baca");
        assert_eq!(encrypt(String::from("karaca")), "0c0r0kaca");
        assert_eq!(encrypt(String::from("burak")), "k0r3baca");
        assert_eq!(encrypt(String::from("alpaca")), "0c0pl0aca");
        assert_eq!(encrypt(String::from("hello")), "2ll1haca");
    }
}