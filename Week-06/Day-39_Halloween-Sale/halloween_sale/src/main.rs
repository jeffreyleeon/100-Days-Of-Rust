fn max_games(p: i32, d: i32, m: i32, s: i32) -> i32 {
    let mut games = 0;
    let mut price = p;
    let mut budget = s;
    while budget >= price {
        games += 1;
        budget -= price;
        price = (price - d).max(m);
    }
    games
}

fn main() {
    let result = max_games(20, 3, 6, 80);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_games() {
        assert_eq!(max_games(20, 3, 6, 80), 6);
        assert_eq!(max_games(20, 3, 6, 85), 7);
        assert_eq!(max_games(20, 3, 6, 19), 0);
    }
}
