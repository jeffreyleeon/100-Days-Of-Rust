fn time_in_words(h: u32, m: u32) -> String {
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen", "twenty", "twenty one", "twenty two", "twenty three", "twenty four",
        "twenty five", "twenty six", "twenty seven", "twenty eight", "twenty nine"
    ];

    if m == 0 {
        return format!("{} o' clock", numbers[h as usize]);
    } else if m == 15 {
        return format!("quarter past {}", numbers[h as usize]);
    } else if m == 30 {
        return format!("half past {}", numbers[h as usize]);
    } else if m == 45 {
        return format!("quarter to {}", numbers[(h % 12 + 1) as usize]);
    }

    let (minutes, direction, hour) = if m <= 30 {
        (m, "past", h)
    } else {
        (60 - m, "to", (h % 12) + 1)
    };

    let minute_word = if minutes == 1 {
        "minute".to_string()
    } else {
        "minutes".to_string()
    };

    format!("{} {} {} {}", numbers[minutes as usize], minute_word, direction, numbers[hour as usize])
}

fn main() {
    println!("{}", time_in_words(5, 47));  // thirteen minutes to six
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_in_words() {
        assert_eq!(time_in_words(3, 0), "three o' clock");
        assert_eq!(time_in_words(7, 15), "quarter past seven");
        assert_eq!(time_in_words(5, 30), "half past five");
        assert_eq!(time_in_words(5, 1), "one minute past five");
    }
}