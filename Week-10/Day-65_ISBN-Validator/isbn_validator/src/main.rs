fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut weight = 10;
    for c in isbn.chars() {
        if c == '-' {
            continue;
        }
        if c == 'X' && weight == 1 {
            sum += 10;
        } else if c == 'X' {
            return false;
        } else if c.is_digit(10) {
            sum += c.to_digit(10).unwrap() * weight;
        } else {
            return false;
        }
        weight -= 1;
    }
    if weight != 0 {
        return false;
    }
    sum % 11 == 0
}

fn main() {
    let isbn = "0-7475-3269-9";
    println!("Is ISBN {} valid? {}", isbn, is_valid_isbn(isbn));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_isbn() {
        assert_eq!(is_valid_isbn("0-7475-3269-9"), true);
        assert_eq!(is_valid_isbn("0-7475-3269-8"), false);
        assert_eq!(is_valid_isbn("0-7475-3269-X"), false);
        assert_eq!(is_valid_isbn("0-7475-3269-"), false);
        assert_eq!(is_valid_isbn("1-5688-1111-X"), true);
    }
}