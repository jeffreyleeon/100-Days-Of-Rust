use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let nm: Vec<usize> = lines.next().unwrap()?.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m) = (nm[0], nm[1]);

    // Read server information
    let mut ip_to_name = HashMap::new();
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        ip_to_name.insert(parts[1].to_string(), parts[0].to_string());
    }

    // Process and print commands
    for _ in 0..m {
        let line = lines.next().unwrap()?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let ip = parts[1].trim_end_matches(';');
        let name = ip_to_name.get(ip).unwrap();
        println!("{} #{}", line.trim_end(), name);
    }

    Ok(())
}
