use rand;
use rand::Rng;

fn isbn_generator() -> String {
    let mut isbn = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..9 {
        let num = rng.gen_range(0..=9);
        isbn.push_str(&num.to_string());
    }
    let mut sum = 0;
    for (i, c) in isbn.chars().enumerate() {
        sum += (i + 1) as u32 * c.to_digit(10).unwrap();
    }
    let check_digit = sum % 11;
    if check_digit == 10 {
        isbn.push('X');
    } else {
        isbn.push_str(&check_digit.to_string());
    }
    isbn
}

fn main() {
    let isbn = isbn_generator();
    println!("{}", isbn);
}
