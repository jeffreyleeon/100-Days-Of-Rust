fn minutes_before_new_year(hour: i32, minute: i32) -> i32 {
    let hours_remaining = 23 - hour;
    let minutes_remaining = 60 - minute;
    let total_minutes = hours_remaining * 60 + minutes_remaining;
    total_minutes
}

fn main() {
    let hour = 23;
    let minute = 59;
    let minutes_remaining = minutes_before_new_year(hour, minute);
    println!("{}", minutes_remaining);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minutes_before_new_year() {
        assert_eq!(minutes_before_new_year(23, 59), 1);
        assert_eq!(minutes_before_new_year(23, 0), 60);
        assert_eq!(minutes_before_new_year(0, 1), 1439);
        assert_eq!(minutes_before_new_year(4, 20), 1180);
        assert_eq!(minutes_before_new_year(23, 59), 1);
    }
}