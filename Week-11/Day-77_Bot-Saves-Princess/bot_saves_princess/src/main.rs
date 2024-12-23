fn next_move(n: i32, r: i32, c: i32, grid: Vec<String>) -> String {
    let mut princess_row = 0;
    let mut princess_col = 0;

    // Find the princess's position
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.find('p') {
            princess_row = i as i32;
            princess_col = j as i32;
        }
    }

    let mut moves = Vec::new();

    // Calculate the difference in rows and columns
    let row_diff = princess_row - r;
    let col_diff = princess_col - c;

    // Move up or down
    if row_diff < 0 {
        for _ in 0..-row_diff {
            moves.push("UP".to_string());
        }
    } else if row_diff > 0 {
        for _ in 0..row_diff {
            moves.push("DOWN".to_string());
        }
    }

    // Move left or right
    if col_diff < 0 {
        for _ in 0..-col_diff {
            moves.push("LEFT".to_string());
        }
    } else if col_diff > 0 {
        for _ in 0..col_diff {
            moves.push("RIGHT".to_string());
        }
    }

    // Join the moves into a single string
    moves.join(", ")
}

fn main() {
    let n = 5;
    let r = 2;
    let c = 3;
    let grid = vec![
        "-----".to_string(),
        "-----".to_string(),
        "p--m-".to_string(),
        "-----".to_string(),
        "-----".to_string(),
    ];

    let result = next_move(n, r, c, grid);
    println!("{}", result);
}
