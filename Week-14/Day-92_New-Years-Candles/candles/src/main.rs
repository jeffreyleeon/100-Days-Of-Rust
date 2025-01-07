fn count_candles(total_new_candles: u32, num_candles_to_make_new_candle: u32) -> u32 {
    let mut total_hours = total_new_candles;
    let mut new_candles_from_old = total_new_candles / num_candles_to_make_new_candle;
    total_hours += new_candles_from_old;
    while new_candles_from_old >= num_candles_to_make_new_candle {
        new_candles_from_old = new_candles_from_old / num_candles_to_make_new_candle;
        total_hours += new_candles_from_old;
    }
    total_hours
}

fn main() {
    let total_new_candles = 4;
    let num_candles_to_make_new_candle = 2;
    let result = count_candles(total_new_candles, num_candles_to_make_new_candle);
    println!("Total hours: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_candles() {
        assert_eq!(count_candles(6, 3), 8);
        assert_eq!(count_candles(12, 4), 15);
        assert_eq!(count_candles(17, 11), 18);
    }
}