fn min_turns(n: i32, p: i32) -> i32 {
    let front = p / 2;
    let back = n / 2 - front;
    front.min(back)
}

fn main() {
    let n = 6;
    let p = 2;
    let result = min_turns(n, p);
    println!("Minimum number of turns: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_turns() {
        assert_eq!(min_turns(6, 2), 1);
        assert_eq!(min_turns(5, 4), 0);
        assert_eq!(min_turns(5, 3), 1);
    }
}