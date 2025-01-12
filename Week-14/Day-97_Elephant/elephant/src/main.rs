fn total_steps(n: i32) -> i32 {
    if n % 5 == 0 {
        n / 5
    } else {
        n / 5 + 1
    }
}

fn main() {
    println!("{}", total_steps(12));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_steps() {
        assert_eq!(total_steps(12), 3);
        assert_eq!(total_steps(16), 4);
        assert_eq!(total_steps(5), 1);
        assert_eq!(total_steps(6), 2);
    }
}