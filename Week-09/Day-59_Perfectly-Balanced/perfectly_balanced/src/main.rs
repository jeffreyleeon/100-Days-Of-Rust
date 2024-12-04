use std::collections::HashMap;

fn balanced_bonus(s: &str) -> bool {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        char_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    println!("{:?}", char_count.values().collect::<Vec<_>>());
    if char_count.is_empty() {
        return true;
    }
    // Non-empty string
    let reference_value = char_count.values().next().unwrap();
    char_count.values().all(|v| v == reference_value)
}

fn main() {
    let s = "xxxyyyzzz";
    println!("Is balanced: {}", balanced_bonus(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_bonus() {
        assert_eq!(balanced_bonus("xxxyyyzzz"), true);
        assert_eq!(balanced_bonus("abccbaabccba"), true);
        assert_eq!(balanced_bonus("xxxyyyzzzz"), false);
        assert_eq!(balanced_bonus("abcdefghijklmnopqrstuvwxyz"), true);
        assert_eq!(balanced_bonus("pqq"), false);
        assert_eq!(balanced_bonus("fdedfdeffeddefeeeefddf"), false);
        assert_eq!(balanced_bonus("www"), true);
        assert_eq!(balanced_bonus("x"), true);
        assert_eq!(balanced_bonus(""), true);
    }
}