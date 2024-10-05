fn can_fit(mut items: Vec<u8>, num_bags: u8) -> bool {
    let mut bags = vec![0; num_bags as usize];
    items.sort_by(|a, b| b.cmp(a));
    for item in items {
        // Find the bag with the most remaining space
        bags.sort();
        println!("{:?}", bags);
        println!("    Addomg item: {:?}", item);
        if bags[0] + item > 10 {
            // If the item is too big to fit in the bag with the most remaining space, return false
            return false;
        }
        bags[0] += item;
    }
    return true;
}

fn main() {
    let items = vec![2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2];
    let num_bags = 4;
    let result = can_fit(items, num_bags);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_fit() {
        assert_eq!(can_fit(vec![2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2], 4), true);
        assert_eq!(can_fit(vec![2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2], 4), false);
        assert_eq!(can_fit(vec![11], 10), false);
        assert_eq!(can_fit(vec![1; 40], 4), true);
        assert_eq!(can_fit(vec![1; 41], 4), false);
    }
}