fn is_vowel(c: char) -> bool {
    match c.to_lowercase().next().unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn sigilize(input: &str) -> String {
    let mut sigil = String::new();
    for c in input.chars().rev() {
        if c.is_alphabetic() && !is_vowel(c) && !sigil.contains(c) {
            sigil.push(c);
        }
    }
    reverse_string(&sigil).to_uppercase()
}

fn main() {
    let result = sigilize("i am healthy");
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigilize() {
        assert_eq!(sigilize("I FOUND MY SOULMATE"), "FNDYSLMT");
        assert_eq!(sigilize("I have a job I enjoy and it pays well"), "HVBJNDTPYSWL");
    }
}