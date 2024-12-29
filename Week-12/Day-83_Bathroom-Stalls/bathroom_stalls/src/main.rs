use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for case in 1..=t {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: u64 = iter.next().unwrap().parse().unwrap();
        let k: u64 = iter.next().unwrap().parse().unwrap();
        
        let (max, min) = solve(n, k);
        println!("Case #{}: {} {}", case, max, min);
    }
}

fn solve(n: u64, k: u64) -> (u64, u64) {
    let mut left = 0;
    let mut right = n + 1;

    for _ in 1..k {
        let mid = left + (right - left) / 2;
        if mid - left > right - mid {
            right = mid;
        } else {
            left = mid;
        }
    }

    let max = (right - left - 1) / 2;
    let min = if (right - left) % 2 == 0 { max - 1 } else { max };

    (max, min)
}
