fn get_josephus_position(n: i32, k: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    return (get_josephus_position(n - 1, k) + k - 1) % n + 1;
}

fn main() {
    let num_of_soldiers = 41;
    let interval = 3;
    let position = get_josephus_position(num_of_soldiers, interval);
    println!("The safe position is: {}", position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_josephus_position() {
        assert_eq!(get_josephus_position(41, 3), 31);
        assert_eq!(get_josephus_position(35, 11), 18);
        assert_eq!(get_josephus_position(11, 1), 11);
        assert_eq!(get_josephus_position(2, 2), 1);
    }
}