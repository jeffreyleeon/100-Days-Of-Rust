use std::io::{self, BufRead};

fn is_tidy(n: u64) -> bool {
    let mut prev_digit = 10; // Start with a digit larger than any possible digit
    let mut num = n;
    while num > 0 {
        let digit = num % 10;
        if digit > prev_digit {
            return false;
        }
        prev_digit = digit;
        num /= 10;
    }
    true
}

fn find_last_tidy(n: u64) -> u64 {
    if is_tidy(n) {
        return n;
    }
    for i in (1..=n).rev() {
        if is_tidy(i) {
            return i;
        }
    }
    1 // This should never happen for valid inputs
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap()?.parse().unwrap();

    for case in 1..=t {
        let n: u64 = lines.next().unwrap()?.parse().unwrap();
        let result = find_last_tidy(n);
        println!("Case #{}: {}", case, result);
    }

    Ok(())
}
