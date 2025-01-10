use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut time_count = HashMap::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let hours: u32 = parts.next().unwrap().parse().unwrap();
        let minutes: u32 = parts.next().unwrap().parse().unwrap();
        let time = hours * 60 + minutes;
        *time_count.entry(time).or_insert(0) += 1;
    }

    let max_cashes = time_count.values().max().unwrap_or(&0);
    println!("{}", max_cashes);
}
