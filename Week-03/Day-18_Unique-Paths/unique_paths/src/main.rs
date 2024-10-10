fn count_unique_paths(m: u128, n: u128) -> u128 {
    // Recursive
    /*
    if m == 1 || n == 1 {
        return 1;
    }
    count_unique_paths(m - 1, n) + count_unique_paths(m, n - 1)
    */

    // Dynamic Programming
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![1; n]; m];

    for i in 1..m {
        for j in 1..n {
            println!("i: {}, j: {}", i, j);
            dp[i][j] = dp[i-1][j] + dp[i][j-1];
        }
    }

    dp[m-1][n-1]
}

fn main() {
    let (m, n) = (25, 25);
    println!("Unique paths: {}", count_unique_paths(m, n));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_unique_paths() {
        assert_eq!(count_unique_paths(3, 7), 28);
        assert_eq!(count_unique_paths(3, 2), 3);
        assert_eq!(count_unique_paths(7, 3), 28);
        assert_eq!(count_unique_paths(3, 3), 6);
        assert_eq!(count_unique_paths(3, 4), 10);
        assert_eq!(count_unique_paths(4, 3), 10);
        assert_eq!(count_unique_paths(4, 4), 20);
        assert_eq!(count_unique_paths(4, 5), 35);
        assert_eq!(count_unique_paths(5, 4), 35);
        assert_eq!(count_unique_paths(5, 5), 70);
        assert_eq!(count_unique_paths(5, 6), 126);
        assert_eq!(count_unique_paths(6, 5), 126);
        assert_eq!(count_unique_paths(6, 6), 252);
        assert_eq!(count_unique_paths(6, 7), 462);
        assert_eq!(count_unique_paths(7, 6), 462);
        assert_eq!(count_unique_paths(7, 7), 924);
        assert_eq!(count_unique_paths(7, 8), 1716);
        assert_eq!(count_unique_paths(8, 7), 1716);
        assert_eq!(count_unique_paths(8, 8), 3432);
        assert_eq!(count_unique_paths(8, 9), 6435);
        assert_eq!(count_unique_paths(9, 8), 6435);
        assert_eq!(count_unique_paths(9, 9), 12870);
        assert_eq!(count_unique_paths(9, 10), 24310);
        assert_eq!(count_unique_paths(10, 9), 24310);
        assert_eq!(count_unique_paths(10, 10), 48620);
        assert_eq!(count_unique_paths(10, 11), 92378);
        assert_eq!(count_unique_paths(11, 10), 92378);
        assert_eq!(count_unique_paths(11, 11), 184756);
        assert_eq!(count_unique_paths(11, 12), 352716);
    }
}