use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for case in 1..=t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut matrix = vec![vec![0; n]; n];
        
        for i in 0..n {
            matrix[i] = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
        }

        let (trace, row_repeats, col_repeats) = solve(&matrix);
        println!("Case #{}: {} {} {}", case, trace, row_repeats, col_repeats);
    }
}

fn solve(matrix: &Vec<Vec<i32>>) -> (i32, i32, i32) {
    let n = matrix.len();
    let mut trace = 0;
    let mut row_repeats = 0;
    let mut col_repeats = 0;

    for i in 0..n {
        trace += matrix[i][i];
        
        if has_repeats(&matrix[i]) {
            row_repeats += 1;
        }

        let column: Vec<i32> = (0..n).map(|j| matrix[j][i]).collect();
        if has_repeats(&column) {
            col_repeats += 1;
        }
    }

    (trace, row_repeats, col_repeats)
}

fn has_repeats(vec: &Vec<i32>) -> bool {
    let mut set = HashSet::new();
    vec.iter().any(|&x| !set.insert(x))
}
