fn calculate_unique_bst(n: u32) -> u32 {
    let mut dp: Vec<u32> = vec![0; ((n + 1)).try_into().unwrap()];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        println!("Calculating BST with {} nodes in total.", i);
        for j in 1..=i {
            let left_count: u32 = dp[(j-1) as usize]; // j-1 is number of nodes on the left, dp[j-1] gives # of unique BST on the left.
            let right_count: u32 = dp[(i-j) as usize]; // i-j is number of nodes on the right, dp[i-j] gives # of unique BST on the right.
            println!("    Consider {} as root; # of left is {}; # of right is {}", j, &left_count, &right_count);
            dp[i as usize] += left_count * right_count; // Adding products of left and right count
        }
        println!("        Got the number of unique BST with {} nodes: {}.", i, dp[i as usize]);
    }
    dp[n as usize]
}

fn main() {
    let input: u32 = 4;
    let result = calculate_unique_bst(input);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_possible_inputs() {
        let mut input: u32 = 1;
        let mut result: u32 = calculate_unique_bst(input);
        assert_eq!(result, 1);
        
        input = 2;
        result = calculate_unique_bst(input);
        assert_eq!(result, 2);
        
        input = 3;
        result = calculate_unique_bst(input);
        assert_eq!(result, 5);
        
        input = 4;
        result = calculate_unique_bst(input);
        assert_eq!(result, 14);
        
        input = 5;
        result = calculate_unique_bst(input);
        assert_eq!(result, 42);
        
        input = 6;
        result = calculate_unique_bst(input);
        assert_eq!(result, 132);
        
        input = 7;
        result = calculate_unique_bst(input);
        assert_eq!(result, 429);
        
        input = 8;
        result = calculate_unique_bst(input);
        assert_eq!(result, 1430);
        
        input = 9;
        result = calculate_unique_bst(input);
        assert_eq!(result, 4862);
        
        input = 10;
        result = calculate_unique_bst(input);
        assert_eq!(result, 16796);
        
        input = 11;
        result = calculate_unique_bst(input);
        assert_eq!(result, 58786);
        
        input = 12;
        result = calculate_unique_bst(input);
        assert_eq!(result, 208012);
        
        input = 13;
        result = calculate_unique_bst(input);
        assert_eq!(result, 742900);
        
        input = 14;
        result = calculate_unique_bst(input);
        assert_eq!(result, 2674440);
        
        input = 15;
        result = calculate_unique_bst(input);
        assert_eq!(result, 9694845);
        
        input = 16;
        result = calculate_unique_bst(input);
        assert_eq!(result, 35357670);
        
        input = 17;
        result = calculate_unique_bst(input);
        assert_eq!(result, 129644790);
        
        input = 18;
        result = calculate_unique_bst(input);
        assert_eq!(result, 477638700);
        
        input = 19;
        result = calculate_unique_bst(input);
        assert_eq!(result, 1767263190);
    }
}