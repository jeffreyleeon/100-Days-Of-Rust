fn keyboard_mistake_fixer(s: &str) -> String {
    let mut fixed = String::new();
    let mut wertyu = vec![
        "1234567890-=".to_string(),
        "QWERTYUIOP[]".to_string(),
        "ASDFGHJKL;'".to_string(),
        "ZXCVBNM,./".to_string(),
    ];
    for c in s.chars() {
        let mut found = false;
        for (i, row) in wertyu.iter().enumerate() {
            if let Some(pos) = row.find(c) {
                fixed.push(wertyu[i].chars().nth(pos - 1).unwrap());
                found = true;
                break;
            }
        }
        if !found {
            fixed.push(c);
        }
    }
    fixed
}

fn main() {
    let s = "O S, GOMR YPFSU/";
    let fixed = keyboard_mistake_fixer(s);
    println!("{}", fixed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_mistake_fixer() {
        assert_eq!(keyboard_mistake_fixer("O S, GOMR YPFSU/"), "I AM FINE TODAY.");
        assert_eq!(keyboard_mistake_fixer("YJR;P"), "THELO");
        assert_eq!(keyboard_mistake_fixer("J;"), "HL");
    }
}