fn breaking_records(scores: &[i32]) -> [i32; 2] {
    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        } else if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    [max_breaks, min_breaks]
}

fn main() {
    let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
    let result = breaking_records(&scores);
    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_breaking_records() {
        assert_eq!(breaking_records(&[12, 24, 10, 24]), [1, 1]);
    }
}