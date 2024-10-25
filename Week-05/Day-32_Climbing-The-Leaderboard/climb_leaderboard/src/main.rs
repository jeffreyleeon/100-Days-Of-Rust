fn player_rank(ranked_scores: Vec<i32>, player_scores: Vec<i32>) -> Vec<i32> {
    // Step 1: Remove duplicates and sort in descending order
    let mut unique_ranked_scores: Vec<i32> = ranked_scores.into_iter().collect();
    unique_ranked_scores.sort_unstable_by(|a, b| b.cmp(a));
    unique_ranked_scores.dedup();

    // Step 2: Find the rank for each player score
    let mut ranks = Vec::new();
    for score in player_scores {
        let rank = unique_ranked_scores.partition_point(|&x| x > score) + 1;
        ranks.push(rank as i32);
    }

    // Step 3: Return the vector of ranks
    ranks
}

fn main() {
    let ranked_scores = vec![100, 90, 90, 80, 75, 60];
    let player_scores = vec![50, 77, 65];
    let ranks = player_rank(ranked_scores, player_scores);
    println!("{:?}", ranks);
}
