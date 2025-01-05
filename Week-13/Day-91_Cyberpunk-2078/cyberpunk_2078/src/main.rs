fn get_month(current_month: String, delta: u8) -> String {
    let months = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let current_month_index = months.iter().position(|&r| r == current_month).unwrap();
    let new_month_index = (current_month_index as u8 + delta) % 12;
    months[new_month_index as usize].to_string()
}

fn main() {
    let current_month = "January".to_string();
    let delta = 3;
    let new_month = get_month(current_month, delta);
    println!("New month is: {}", new_month);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_month() {
        assert_eq!(get_month("January".to_string(), 3), "April".to_string());
        assert_eq!(get_month("January".to_string(), 12), "January".to_string());
        assert_eq!(get_month("January".to_string(), 13), "February".to_string());
        assert_eq!(get_month("January".to_string(), 14), "March".to_string());
        assert_eq!(get_month("January".to_string(), 15), "April".to_string());
        assert_eq!(get_month("January".to_string(), 16), "May".to_string());
        assert_eq!(get_month("January".to_string(), 17), "June".to_string());
        assert_eq!(get_month("January".to_string(), 18), "July".to_string());
        assert_eq!(get_month("January".to_string(), 19), "August".to_string());
        assert_eq!(get_month("January".to_string(), 20), "September".to_string());
        assert_eq!(get_month("January".to_string(), 21), "October".to_string());
        assert_eq!(get_month("January".to_string(), 22), "November".to_string());
        assert_eq!(get_month("January".to_string(), 23), "December".to_string());
        assert_eq!(get_month("January".to_string(), 24), "January".to_string());
    }
}