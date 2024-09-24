fn count_skewers(skewers: &Vec<&str>) -> (usize, usize) {
    let mut vegetarian_count = 0;
    let mut non_vegetarian_count = 0;
    for skewer in skewers {
        if skewer.contains("-x") {
            non_vegetarian_count += 1;
        } else {
            vegetarian_count += 1;
        }
    }
    return (vegetarian_count, non_vegetarian_count);
}

fn main() {
    let skewers: Vec<&str> = vec![
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ];
    let result = count_skewers(&skewers);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_skewers() {
        let skewers = vec![
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",
        ];
        let result = count_skewers(&skewers);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_count_skewers_2() {
        let skewers = vec![
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];
        let result = count_skewers(&skewers);
        assert_eq!(result, (2, 3));
    }

    #[test]
    fn test_count_skewers_3() {
        let skewers = vec![
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ];
        let result = count_skewers(&skewers);
        assert_eq!(result, (3, 2));
    }
}