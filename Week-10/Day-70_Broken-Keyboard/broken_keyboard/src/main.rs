use std::fs;
use std::path::Path;

fn read_dictionary_file() -> String {
    let assets_dir = Path::new("../../../assets");
    let dictionary_file = assets_dir.join("dictionary1.txt");
    fs::read_to_string(dictionary_file).expect("Failed to read dictionary file")
}

fn can_form_word(word: &str, keys: &str) -> bool {
    let keys_set: std::collections::HashSet<char> = keys.chars().collect();
    word.chars().all(|c| keys_set.contains(&c))
}

fn longest_word(dictionary: &str, keys: &str) -> Option<String> {
    let words: Vec<&str> = dictionary.lines().collect();
    let mut longest = None;
    let mut max_length = 0;

    for word in words {
        if can_form_word(word, keys) && word.len() > max_length {
            longest = Some(word.to_string());
            max_length = word.len();
        }
    }

    longest
}

fn main() {
    let dictionary_content = read_dictionary_file();
    // Process the dictionary content here
    // println!("{}", dictionary_content);

    let keys = "subtoxymerhlac";
    let result = longest_word(&dictionary_content, keys);
    match result {
        Some(word) => {
            println!("longestWord(\"{}\") ➞ \"{}\"", keys, word);
        }
        None => {
            println!("No word found for keys: {}", keys);
        }
    }
    
}
