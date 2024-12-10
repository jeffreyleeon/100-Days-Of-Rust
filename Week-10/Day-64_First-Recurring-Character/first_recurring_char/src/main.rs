fn get_first_recurring_char(s: &str) -> Option<char> {
    let mut char_set = std::collections::HashSet::new();
    for c in s.chars() {
        if char_set.contains(&c) {
            return Some(c);
        }
        char_set.insert(c);
    }
    None
}

fn main() {
    println!("{:?}", get_first_recurring_char("ABCA"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_recurring_char() {
        assert_eq!(get_first_recurring_char("ABCA"), Some('A'));
        assert_eq!(get_first_recurring_char("BCABA"), Some('B'));
        assert_eq!(get_first_recurring_char("ABCDEBC"), Some('B'));
        assert_eq!(get_first_recurring_char("ABC"), None);
    }
}