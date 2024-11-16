fn contain_keyword_from_list(text: &str, keywords: Vec<&str>) -> bool {
    for keyword in keywords {
        if text.contains(keyword) {
            return true;
        }
    }
    false
}

fn main() {
    let keywords = vec![
        "anime",
        "meme",
        "vines",
        "roasts",
        "Danny DeVito",
    ];
    let text = "vines that butter my eggroll";
    if contain_keyword_from_list(text, keywords) {
        println!("NO!");
    } else {
        println!("Safe watching!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contain_keyword_from_list() {
        let keywords = vec![
            "anime",
            "meme",
            "vines",
            "roasts",
            "Danny DeVito",
        ];
        assert_eq!(contain_keyword_from_list("anime", keywords.clone()), true);
        assert_eq!(contain_keyword_from_list("meme", keywords.clone()), true);
        assert_eq!(contain_keyword_from_list("vines", keywords.clone()), true);
        assert_eq!(contain_keyword_from_list("roasts", keywords.clone()), true);
        assert_eq!(contain_keyword_from_list("Danny DeVito", keywords.clone()), true);
        assert_eq!(contain_keyword_from_list("Hot pictures of Danny DeVito", keywords.clone()), true);
        assert_eq!(contain_keyword_from_list("How to ace BC Calculus in 5 Easy Steps", keywords.clone()), false);
    }
}
