/// Checks if a given number is prime.
///
/// A prime number is a natural number greater than 1 that cannot be formed by multiplying two smaller natural numbers.
/// This function returns `true` if the number is prime, and `false` otherwise.
///
/// # Arguments
///
/// * `num` - A u8 integer to check for primality.
///
/// # Returns
///
/// * `bool` - Returns `true` if the number is prime, `false` otherwise.
///
/// # Examples
///
/// ```
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(4), false);
/// ```
fn is_prime(num: u8) -> bool {
    if num < 2 {
        return false;
    }
    let square_root: u8 = ((num as f64).sqrt().floor()) as u8;
    for i in 2..=square_root {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn get_next_prime(num: u8) -> u8 {
    let mut next_num = num;
    while !is_prime(next_num) {
        next_num += 1;
    }
    next_num
}

fn main() {
    let result: u8 = get_next_prime(13);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_prime() {
        let num = 24;
        let result: u8 = get_next_prime(num);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_get_next_prime_return_itself() {
        let num = 11;
        let result: u8 = get_next_prime(num);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_get_next_prime_small() {
        let mut num = 0;
        let mut result: u8 = get_next_prime(num);
        assert_eq!(result, 2);

        num = 1;
        result = get_next_prime(num);
        assert_eq!(result, 2);

        num = 2;
        result = get_next_prime(num);
        assert_eq!(result, 2);

        num = 3;
        result = get_next_prime(num);
        assert_eq!(result, 3);

        num = 4;
        result = get_next_prime(num);
        assert_eq!(result, 5);
    }
}