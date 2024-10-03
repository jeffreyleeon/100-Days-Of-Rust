fn backtrack(s: &str, start: usize, current: &mut Vec<String>, result: &mut Vec<String>) {
    if current.len() == 4 && start == s.len() {
        // There are 4 parts and used up all the digits.
        result.push(current.join("."));
        return;
    }

    if current.len() == 4 || start == s.len() {
        // Either there are already 4 parts or already used up all digits.
        // Simply return.
        return;
    }

    for i in 1..=3 {
        if start + i > s.len() {
            // Used up all digits.
            break;
        }

        let part = &s[start..start + i];
        if (part.starts_with('0') && part.len() > 1) || part.parse::<u8>().is_err() {
            // Skip if there are leading zero or the part doesn't fit into 8bits (0-255)
            continue;
        }

        current.push(part.to_string());
        backtrack(s, start + i, current, result);
        current.pop(); // Need to pop for next iteration/set of digit.
    }
}

fn main() {
    let s: String = String::from("101023");
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack(&s, 0, &mut current, &mut result);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_ip() {
        let s: String = String::from("25525511135");
        let mut result = Vec::new();
        let mut current = Vec::new();
        backtrack(&s, 0, &mut current, &mut result);
        assert_eq!(result, vec!["255.255.11.135","255.255.111.35"]);
    }

    #[test]
    fn test_restore_ip_2() {
        let s: String = String::from("0000");
        let mut result = Vec::new();
        let mut current = Vec::new();
        backtrack(&s, 0, &mut current, &mut result);
        assert_eq!(result, vec!["0.0.0.0"]);
    }

    #[test]
    fn test_restore_ip_3() {
        let s: String = String::from("1111");
        let mut result = Vec::new();
        let mut current = Vec::new();
        backtrack(&s, 0, &mut current, &mut result);
        assert_eq!(result, vec!["1.1.1.1"]);
    }

    #[test]
    fn test_restore_ip_4() {
        let s: String = String::from("010010");
        let mut result = Vec::new();
        let mut current = Vec::new();
        backtrack(&s, 0, &mut current, &mut result);
        assert_eq!(result, vec!["0.10.0.10","0.100.1.0"]);
    }
}