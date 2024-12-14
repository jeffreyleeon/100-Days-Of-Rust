fn get_pythagoras_distance(a: (i32, i32), b: (i32, i32)) -> f64 {
    println!("{:?} {:?}", a, b);
    let x = (a.0 - b.0).abs();
    let y = (a.1 - b.1).abs();
    ((x as f64).powf(2.0) + (y as f64).powf(2.0)).sqrt()
}

fn total_distance(s: &str) -> f64 {
    let mut distance: f64 = 0.0;
    let digit_position = vec![
        (0, 1),
        (3, 0),
        (3, 1),
        (3, 2),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 0),
        (1, 1),
        (1, 2),
    ];
    let dot_position = (0, 0);
    let mut current_position = None;
    for c in s.chars() {
        let position = match c {
            '0'..='9' => digit_position[c as usize - '0' as usize],
            '.' => dot_position,
            _ => continue,
        };
        if current_position.is_none() {
            current_position = Some(position);
            continue;
        } else {
            distance += get_pythagoras_distance(current_position.unwrap(), position);
        }
        current_position = Some(position);
    }
    distance
}

fn main() {
    let s = "7851";
    println!("{}", total_distance(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        assert_eq!(total_distance("1236547890"), 9.414213562373095);
    }
}