fn is_legitimate(pool: &Vec<Vec<i32>>) -> bool {
    let height = pool.len();
    let width = pool[0].len();
    for i in 0..height {
        for j in 0..width {
            if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                // edge
                if pool[i][j] == 1 {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let pool = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];
    println!("{}", is_legitimate(&pool));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_legitimate() {
        let pool = vec![
            vec![1, 1],
            vec![1, 1],
        ];
        assert_eq!(is_legitimate(&pool), false);

        let pool = vec![
            vec![0, 0],
            vec![0, 0],
        ];
        assert_eq!(is_legitimate(&pool), true);
    }
}