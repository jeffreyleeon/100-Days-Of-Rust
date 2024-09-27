use std::collections::HashMap;

fn count_pairs_of_socks(socks: &str) -> u8 {
    if socks.len() == 0 {
        return 0;
    }
    let mut count: u8 = 0;
    let mut dictionary: HashMap<char, bool> = HashMap::new();
    for c in socks.chars() {
        if dictionary.contains_key(&c) {
            count += 1;
            dictionary.remove(&c);
        } else {
            dictionary.insert(c, true);
        }
    }
    count
}

fn main() {
    let socks = "CABBACCC";
    let result = count_pairs_of_socks(socks);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_pairs_of_socks() {
        let socks = "AA";
        let result = count_pairs_of_socks(socks);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_pairs_of_socks_2() {
        let socks = "ABABC";
        let result = count_pairs_of_socks(socks);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_pairs_of_socks_empty() {
        let socks = "";
        let result = count_pairs_of_socks(socks);
        assert_eq!(result, 0);
    }
}