use std::collections::HashMap;
use std::cmp;

fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut task_counts = HashMap::new();
    let mut max_count = 0;
    let mut max_count_tasks = 0;

    // Count the frequency of each task
    for &task in &tasks {
        let count = task_counts.entry(task).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            max_count_tasks = 1;
        } else if *count == max_count {
            max_count_tasks += 1;
        }
    }

    // Calculate the minimum number of units required
    let mut result = (max_count - 1) * (n + 1) + max_count_tasks;
    
    // Return the maximum of the calculated result and the total number of tasks
    cmp::max(result, tasks.len() as i32)
}

fn main() {
    let tasks1 = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n1 = 2;
    println!("Example 1: {}", least_interval(tasks1, n1)); // Output: 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_interval() {
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'], 2), 16);
    }
}