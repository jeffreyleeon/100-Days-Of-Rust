use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of paintings
    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the beauty values of paintings
    let beauties: HashSet<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // The maximum number of times a visitor becomes happy is one less than the number of unique beauty values
    println!("{}", beauties.len() - 1);
}
