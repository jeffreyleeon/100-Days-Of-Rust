fn get_money_spent(keyboards: &[i32], drives: &[i32], b: i32) -> i32 {
    let mut max_spent = -1;

    for &keyboard_price in keyboards {
        for &drive_price in drives {
            let total_cost = keyboard_price + drive_price;
            if total_cost <= b && total_cost > max_spent {
                max_spent = total_cost;
            }
        }
    }

    max_spent
}

fn main() {
    let keyboards = vec![40, 50, 60];
    let drives = vec![5, 8, 12];
    let budget = 60;

    let result = get_money_spent(&keyboards, &drives, budget);
    println!("Maximum amount that can be spent: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_money_spent() {
        assert_eq!(get_money_spent(&[3, 1], &[5, 2, 8], 10), 9);
        assert_eq!(get_money_spent(&[4], &[5], 5), -1);
    }
}