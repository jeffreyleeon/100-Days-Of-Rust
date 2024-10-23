fn max_possible_value(n: i32, digit: i32) -> i32 {
    let is_negative = n < 0;
    let mut n_str = n.abs().to_string();
    let digit_char = char::from_digit(digit as u32, 10).unwrap();
    let mut max_value = if is_negative { i32::MAX } else { i32::MIN };

    for i in 0..=n_str.len() {
        let mut new_str = n_str.clone();
        new_str.insert(i, digit_char);
        if let Ok(new_num) = new_str.parse::<i32>() {
            if is_negative {
                max_value = max_value.min(new_num);
            } else {
                max_value = max_value.max(new_num);
            }
        }
    }

    if is_negative { -max_value } else { max_value }
}

fn main() {
    println!("{}", max_possible_value(276, 3));   // Output: 3276
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_possible_value() {
        assert_eq!(max_possible_value(276, 3), 3276);
        assert_eq!(max_possible_value(-999, 4), -4999);
        assert_eq!(max_possible_value(0, 3), 30);
        assert_eq!(max_possible_value(860, 7), 8760);
    }
}