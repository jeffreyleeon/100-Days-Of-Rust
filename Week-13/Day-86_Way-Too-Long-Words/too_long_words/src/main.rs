fn abbreviate_words(word: &str) -> String {
    let mut result = String::new();
    if word.len() <= 10 {
        // No need to abbreviate
        return word.to_string();
    }
    result.push(word.chars().nth(0).unwrap());
    result.push_str(&(word.len() - 2).to_string());
    result.push(word.chars().nth(word.len() - 1).unwrap());
    result
}

fn main() {
    println!("{}", abbreviate_words("localization"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbreviate_words() {
        assert_eq!(abbreviate_words("word"), "word");
        assert_eq!(abbreviate_words("localization"), "l10n");
        assert_eq!(abbreviate_words("internationalization"), "i18n");
        assert_eq!(abbreviate_words("pneumonoultramicroscopicsilicovolcanoconiosis"), "p43s");
    }
}