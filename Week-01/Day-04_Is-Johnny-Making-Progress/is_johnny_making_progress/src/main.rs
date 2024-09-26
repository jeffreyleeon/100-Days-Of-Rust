fn count_number_of_progress(johnny_progress: Vec<u8>) -> u8 {
    if johnny_progress.len() == 0 {
        return 0;
    }
    // Count the number of progress
    let mut count: u8 = 0;
    let mut baseline: u8 = johnny_progress[0];
    for progress in johnny_progress {
        if progress > baseline {
            count += 1;
        }
        baseline = progress;
    }
    count
}

fn main() {
    let johnny_progress: Vec<u8> = vec![3, 4, 1, 2];
    let result = count_number_of_progress(johnny_progress);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_number_of_progress() {
        let johnny_progress = vec![10, 11, 12, 9, 10];
        let result = count_number_of_progress(johnny_progress);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_number_of_progress_2() {
        let johnny_progress = vec![6, 5, 4, 3, 2, 9];
        let result = count_number_of_progress(johnny_progress);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_number_of_progress_3() {
        let johnny_progress = vec![9, 9];
        let result = count_number_of_progress(johnny_progress);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_number_of_progress_empty() {
        let johnny_progress = vec![];
        let result = count_number_of_progress(johnny_progress);
        assert_eq!(result, 0);
    }

}
