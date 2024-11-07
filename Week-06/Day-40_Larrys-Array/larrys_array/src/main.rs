fn larrys_array(a: &[i32]) -> bool {
    let n = a.len();
    let mut inversions = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            if a[i] > a[j] {
                inversions += 1;
            }
        }
    }

    inversions % 2 == 0
}

fn main() {
    let array = vec![1, 6, 5, 2, 4, 3];
    let result = larrys_array(&array);
    println!("Can be sorted: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_larrys_array() {
        assert_eq!(larrys_array(&[1, 6, 5, 2, 4, 3]), true);
        assert_eq!(larrys_array(&[3, 1, 2]), true);
        assert_eq!(larrys_array(&[1, 3, 2, 4]), false);
    }
}