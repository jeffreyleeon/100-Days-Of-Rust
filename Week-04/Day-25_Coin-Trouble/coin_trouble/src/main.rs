fn coins_div(coins: &[i32]) -> bool {
    let total: i32 = coins.iter().sum();
    
    // If the total is not divisible by 3, it's impossible to distribute equally
    if total % 3 != 0 {
        return false;
    }
    
    let target = total / 3;
    let n = coins.len();
    
    // Create a 3D DP table
    let mut dp = vec![vec![vec![false; target as usize + 1]; n + 1]; 4];
    
    // Initialize base case
    dp[0][0][0] = true;
    
    // Fill the DP table
    for i in 1..=3 {
        for j in 1..=n {
            for k in 0..=target as usize {
                // Don't take the current coin
                dp[i][j][k] = dp[i][j-1][k];
                
                // Take the current coin if possible
                if k >= coins[j-1] as usize {
                    dp[i][j][k] |= dp[i-1][j-1][k - coins[j-1] as usize];
                }
            }
        }
    }
    
    // Check if it's possible to distribute equally
    dp[3][n][target as usize]
}

fn main() {
    println!("{}", coins_div(&[1, 2, 3, 2, 2, 2, 3])); // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coins_div() {
        assert_eq!(coins_div(&[5, 3, 10, 1, 2]), false);
        assert_eq!(coins_div(&[2, 4, 3, 2, 4, 9, 7, 8, 6, 9]), true);
    }
}