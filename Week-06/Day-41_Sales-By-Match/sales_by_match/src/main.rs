use std::collections::HashMap;

fn sock_merchant(ar: &[i32]) -> i32 {
    let mut color_count = HashMap::new();
    
    // Count occurrences of each color
    for &color in ar {
        *color_count.entry(color).or_insert(0) += 1;
    }
    
    // Calculate pairs
    let pairs = color_count.values()
        .map(|&count| count / 2)
        .sum();
    
    pairs
}

fn main() {
    let n = 7;
    let ar = vec![1, 2, 1, 2, 1, 3, 2];
    let result = sock_merchant(&ar);
    println!("Number of pairs: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 9;
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(&ar), 3);
    }

    #[test]
    fn test_case_2() {
        let n = 10;
        let ar = vec![1, 1, 3, 1, 2, 1, 3, 3, 3, 3];
        assert_eq!(sock_merchant(&ar), 4);
    }
}