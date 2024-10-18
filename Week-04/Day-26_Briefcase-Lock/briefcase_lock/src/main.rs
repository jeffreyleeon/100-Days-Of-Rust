use std::cmp::min;

fn get_min_turns(current: &str, target: &str) -> usize {
    let mut total_turns = 0;
    for (c, t) in current.chars().zip(target.chars()) {
        let turns = min(
            (c as i32 - t as i32).abs(),
            10 - (c as i32 - t as i32).abs(),
        );
        println!("{} -> {} = {}", c, t, turns);
        total_turns += turns as usize;
    }
    total_turns
}

fn main() {
    let current = "4089";
    let target = "5672";
    let result = get_min_turns(current, target);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_min_turns() {
        assert_eq!(get_min_turns("4089", "5673"), 10);
        assert_eq!(get_min_turns("2391", "4984"), 10);
        assert_eq!(get_min_turns("1111", "1100"), 2);
        assert_eq!(get_min_turns("1111", "0000"), 4);
        assert_eq!(get_min_turns("0000", "1111"), 4);
        assert_eq!(get_min_turns("1234", "0000"), 10);
        assert_eq!(get_min_turns("0000", "1234"), 10);
        assert_eq!(get_min_turns("1234", "4321"), 8);
        assert_eq!(get_min_turns("4321", "1234"), 8);
        assert_eq!(get_min_turns("9876", "1234"), 12);
    }
}