fn compose_hulk(n: u8) -> String {
    let mut hulk = String::new();
    for i in 0..n {
        if i % 2 == 0 {
            hulk.push_str("I hate ");
        } else {
            hulk.push_str("I love ");
        }
        if i != n - 1 {
            hulk.push_str("that ");
        }
    }
    hulk.push_str("it");
    hulk
}

fn main() {
    println!("{}", compose_hulk(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_hulk() {
        assert_eq!(compose_hulk(1), "I hate it");
        assert_eq!(compose_hulk(2), "I hate that I love it");
        assert_eq!(compose_hulk(3), "I hate that I love that I hate it");
    }
}