fn free_prisoners(prison: Vec<u8>) -> u8 {
    if prison[0] == 0 {
        return 0;
    }
    let mut freed = 0;
    for i in 0..prison.len() {
        let is_unlocked = (prison[i] + freed) % 2 == 1;
        if is_unlocked {
            freed += 1;
        }
    }
    freed
}

fn main() {
    let prison: Vec<u8> = vec![1, 1, 0, 0, 0, 1, 0];
    println!("Free prisoners: {}", free_prisoners(prison));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_free_prisoners() {
        assert_eq!(free_prisoners(vec![1, 1, 0, 0, 0, 1, 0]), 4);
        assert_eq!(free_prisoners(vec![1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0]), 10);
        assert_eq!(free_prisoners(vec![1, 1, 1]), 1);
        assert_eq!(free_prisoners(vec![0, 0, 0]), 0);
        assert_eq!(free_prisoners(vec![0, 1, 1, 1]), 0);
    }
}