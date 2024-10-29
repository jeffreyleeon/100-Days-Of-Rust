
const DIGITS: [&str; 10] = [
    "-|| ||-",  // 0
    "  |  | ",  // 1
    "- |-| -",  // 2
    "- |- |-",  // 3
    " ||- | ",  // 4
    "-| - |-",  // 5
    "-| -||-",  // 6
    "- |  | ",  // 7
    "-||-||-",  // 8
    "-||- |-",  // 9
];

fn toLCD(n: u64, s: usize) -> String {
    let digits: Vec<char> = n.to_string().chars().collect();
    let mut result = vec![String::new(); 2 * s + 3];

    for digit in digits {
        let d = digit.to_digit(10).unwrap() as usize;
        let segments = DIGITS[d];

        result[0] += &horizontal(if segments.chars().nth(0) == Some('-') { '-' } else { ' ' }, s);
        result[0] += " ";

        for i in 1..=s {
            result[i] += &vertical(
                if segments.chars().nth(1) == Some('|') { '|' } else { ' ' },
                if segments.chars().nth(2) == Some('|') { '|' } else { ' ' },
                s
            );
            result[i] += " ";
        }

        result[s+1] += &horizontal(if segments.chars().nth(3) == Some('-') { '-' } else { ' ' }, s);
        result[s+1] += " ";

        for i in s+2..2*s+2 {
            result[i] += &vertical(
                if segments.chars().nth(4) == Some('|') { '|' } else { ' ' },
                if segments.chars().nth(5) == Some('|') { '|' } else { ' ' },
                s
            );
            result[i] += " ";
        }

        // println!("The 6th: {}", segments.chars().nth(6).unwrap());
        result[2*s+2] += &horizontal(if segments.chars().nth(6) == Some('-') { '-' } else { ' ' }, s);
        result[2*s+2] += " ";
    }
    // println!("Result is {:?}", result);
    result.join("\n")
}

fn horizontal(c: char, s: usize) -> String {
    format!(" {} ", c.to_string().repeat(s))
}

fn vertical(left: char, right: char, s: usize) -> String {
    format!("{}{}{}",
        left,
        " ".repeat(s),
        right
    )
}

fn main() {
    println!("{}", toLCD(1234567890, 2));
}