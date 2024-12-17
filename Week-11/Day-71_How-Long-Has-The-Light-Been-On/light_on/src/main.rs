fn light_on(intervals: Vec<(i32, i32)>) -> i32 {
    // Sort the intervals by the start time
    let mut intervals: Vec<(i32, i32)> = intervals;
    intervals.sort_by_key(|&(start, _)| start);

    // Initialize variables to keep track of the merged intervals
    let mut merged_intervals = Vec::new();
    let mut current_start = None;
    let mut current_end = None;

    for (start, end) in intervals {
        // If this is the first interval or it does not overlap with the current one, start a new interval
        if current_start.is_none() || start > current_end.unwrap_or(0) {
            if let (Some(s), Some(e)) = (current_start, current_end) {
                merged_intervals.push((s, e));
            }
            current_start = Some(start);
            current_end = Some(end);
        } else {
            // Update the end of the current interval if this one extends it
            current_end = Some(current_end.unwrap_or(0).max(end));
        }
    }

    // Add the last interval
    if let (Some(s), Some(e)) = (current_start, current_end) {
        merged_intervals.push((s, e));
    }

    // Calculate the total duration
    let mut total_duration = 0;
    for (start, end) in merged_intervals {
        total_duration += end - start;
    }

    total_duration
}

fn main() {
    let intervals1 = vec![(2, 4), (3, 6), (1, 3), (6, 8)];
    println!("lightOn({:?}) ➞ {}", intervals1.clone(), light_on(intervals1));

    let intervals2 = vec![(6, 8), (5, 8), (8, 9), (5, 7), (4, 7)];
    println!("lightOn({:?}) ➞ {}", intervals2.clone(), light_on(intervals2));
}
