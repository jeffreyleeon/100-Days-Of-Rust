fn minimun_bills(mut n: i32) -> i32 {
    let mut bills = 0;
    let denominations = vec![100, 20, 10, 5, 1];
    for i in 0..denominations.len() {
        bills += n / denominations[i];
        n = n % denominations[i];
    }
    bills
}

fn main() {
    println!("{}", minimun_bills(125));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimun_bills() {
        assert_eq!(minimun_bills(125), 3);
        assert_eq!(minimun_bills(43), 5);
        assert_eq!(minimun_bills(100000000), 1000000);
    }
}