fn funny_plant(num_of_people: i32, num_of_fruits: i32) -> i32 {
    let mut plant = num_of_fruits;
    let mut fruits = 0;
    let mut week = 1;
    loop {
        week += 1;
        // Plant grows by the number of fruits it has
        fruits += plant;
        if fruits >= num_of_people {
            return week;
        }
        // Number of fruits will be used to plant for next week
        plant += fruits;
    }
}

fn main() {
    println!("{}", funny_plant(200, 15));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funny_plant() {
        assert_eq!(funny_plant(200, 15), 5);
        assert_eq!(funny_plant(50000, 1), 14);
        assert_eq!(funny_plant(150000, 250), 9);
    }
}