fn identical_filter(words: Vec<String>) -> Vec<String> {
    words.into_iter().filter(|word| {
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        chars.all(|c| c == first)
    }).collect()
}

fn main() {
    let words = vec![
        "aaaaaa".to_string(),
        "bc".to_string(),
        "d".to_string(),
        "eeee".to_string(),
        "xyz".to_string(),
    ];
    println!("{:?}", identical_filter(words));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_filter() {
        assert_eq!(
            identical_filter(vec![
                "88".to_string(),
                "999".to_string(),
                "22".to_string(),
                "545".to_string(),
                "131".to_string(),
            ]),
            vec!["88".to_string(), "999".to_string(), "22".to_string()]
        );
    }

    #[test]
    fn test_identical_filter_2() {
        assert!(
            identical_filter(vec![
                "xxxxo".to_string(),
                "oxo".to_string(),
                "xox".to_string(),
                "ooxxoo".to_string(),
                "oxo".to_string(),
            ]).is_empty()
        );
    }
}