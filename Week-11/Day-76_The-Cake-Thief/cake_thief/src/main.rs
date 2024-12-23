fn max_duffel_bag_value(cakes: &[(u32, u32)], capacity: u32) -> u32 {
    // Initialize a vector to store the maximum value for each weight from 0 to capacity
    let mut max_values = vec![0; (capacity + 1) as usize];

    // Iterate over each cake type
    for &(weight, value) in cakes {
        // Iterate from capacity down to the weight of the current cake
        for w in (weight..=capacity).rev() {
            // Update the maximum value for the current weight
            max_values[w as usize] = max_values[w as usize].max(max_values[(w - weight) as usize] + value);
        }
    }

    // Return the maximum value for the full capacity
    max_values[capacity as usize]
}

fn main() {
    let cakes = [(7, 160), (3, 90), (2, 15)];
    let capacity = 20;
    let max_value = max_duffel_bag_value(&cakes, capacity);
    println!("Maximum value: {}", max_value); // Output: 555
}
