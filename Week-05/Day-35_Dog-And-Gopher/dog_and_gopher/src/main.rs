use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn can_escape(gopher_pos: (f64, f64), dog_pos: (f64, f64), hole_pos: (f64, f64)) -> bool {
    let gopher_dist = distance(gopher_pos, hole_pos);
    let dog_dist = distance(dog_pos, hole_pos);
    gopher_dist * 2.0 < dog_dist
}

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt()
}

fn gopher_escape_plan(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return vec!["BAD FILE! Missing File.".to_string()],
    };
    let reader = BufReader::new(file);

    let mut results = Vec::new();
    let mut current_set = Vec::new();
    let mut n = 0;
    let mut gopher_pos = (0.0, 0.0);
    let mut dog_pos = (0.0, 0.0);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line.trim().to_string(),
            Err(_) => return vec!["BAD FILE!".to_string()],
        };

        if line.is_empty() {
            if !current_set.is_empty() {
                results.push(process_set(&current_set, n, gopher_pos, dog_pos));
                current_set.clear();
                n = 0;
            }
            continue;
        }

        if n == 0 {
            let parts: Vec<f64> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() != 5 {
                return vec!["BAD FILE! 1st line needs 5 items".to_string()];
            }
            n = parts[0] as usize;
            gopher_pos = (parts[1], parts[2]);
            dog_pos = (parts[3], parts[4]);
        } else {
            current_set.push(line);
        }
    }

    if !current_set.is_empty() {
        results.push(process_set(&current_set, n, gopher_pos, dog_pos));
    }

    results
}

fn process_set(set: &[String], n: usize, gopher_pos: (f64, f64), dog_pos: (f64, f64)) -> String {
    if set.len() != n {
        return "BAD FILE! Number of holes declared doesn't match actual count.".to_string();
    }

    for line in set {
        let parts: Vec<f64> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if parts.len() != 2 {
            return "BAD FILE! Each set needs 2 coordinates".to_string();
        }

        let hole_pos = (parts[0], parts[1]);
        if can_escape(gopher_pos, dog_pos, hole_pos) {
            return format!("The gopher can escape through the hole at ({:.6},{:.6}).", hole_pos.0, hole_pos.1);
        }
    }

    "The gopher cannot escape.".to_string()
}

fn main() {
    // You can test your function here
    let result = gopher_escape_plan("./assets/day-35_sample_1_valid.txt");
    println!("{:?}", result);
}