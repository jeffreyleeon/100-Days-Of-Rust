fn measure_trapped_water(input: Vec<u8>) -> u8 {
    if input.len() == 0 {
        return 0;
    }
    let (mut left, mut right) = (0, input.len() - 1);
    let (mut left_max, mut right_max, mut water) = (0, 0, 0);
    while left < right {
        if input[left] < input[right] {
            if input[left] >= left_max {
                left_max = input[left];
            } else {
                water += left_max - input[left]
            }
            left += 1;
        } else {
            if input[right] >= right_max {
                right_max = input[right];
            } else {
                water += right_max - input[right]
            }
            right -= 1;
        }
    }
    water
}

fn main() {
    let input: Vec<u8> = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let result: u8 = measure_trapped_water(input);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_height() {
        let input: Vec<u8> = vec![];
        let result: u8 = measure_trapped_water(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ascending_height() {
        let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let result: u8 = measure_trapped_water(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_descending_height() {
        let input: Vec<u8> = vec![6, 5, 4, 3, 2, 1];
        let result: u8 = measure_trapped_water(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_peak() {
        let input: Vec<u8> = vec![1, 2, 3, 4, 3, 2 ,1];
        let result: u8 = measure_trapped_water(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_valley() {
        let input: Vec<u8> = vec![4, 3, 2, 1, 2, 3, 4];
        let result: u8 = measure_trapped_water(input);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_normal() {
        let input: Vec<u8> = vec![0, 2, 3, 0, 2, 1, 4, 0, 3, 2, 4, 1, 5, 0];
        let result: u8 = measure_trapped_water(input);
        assert_eq!(result, 16);
    }
}