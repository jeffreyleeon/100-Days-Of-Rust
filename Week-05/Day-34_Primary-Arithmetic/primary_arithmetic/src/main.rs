fn carry_count(a: u64, b: u64) -> u64 {
    let mut carry = 0;
    let mut carry_count = 0;
    let mut a = a;
    let mut b = b;
    while a > 0 || b > 0 {
        let a_digit = a % 10;
        let b_digit = b % 10;
        let sum = a_digit + b_digit + carry;
        if sum >= 10 {
            carry_count += 1;
            carry = 1;
        } else {
            carry = 0;
        }
        a /= 10;
        b /= 10;
    }
    carry_count
}

fn main() {
    println!("{}", carry_count(555, 555)); // 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carry_count() {
        assert_eq!(carry_count(555, 555), 3);
        assert_eq!(carry_count(123, 456), 0);
        assert_eq!(carry_count(555, 545), 3);
        assert_eq!(carry_count(123, 594), 1);
        assert_eq!(carry_count(1, 2000), 0);
        assert_eq!(carry_count(1, 2), 0);
    }
}