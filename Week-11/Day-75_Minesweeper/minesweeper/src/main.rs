use rand::Rng;
use std::collections::HashSet;

// Structure to represent the game board
struct Minesweeper {
    board: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    mines: usize,
}

impl Minesweeper {
    // Function to create a new Minesweeper game
    fn new(rows: usize, cols: usize, mines: usize) -> Self {
        let mut board = vec![vec![0; cols]; rows];
        let mut placed_mines = 0;
        let mut rng = rand::thread_rng();

        // Place mines randomly
        while placed_mines < mines {
            let row = rng.gen_range(0..rows);
            let col = rng.gen_range(0..cols);
            if board[row][col] != 9 {
                board[row][col] = 9;
                placed_mines += 1;
            }
        }

        // Calculate adjacent mine counts
        for row in 0..rows {
            for col in 0..cols {
                if board[row][col] == 9 {
                    continue;
                }
                let mut count = 0;
                for r in (row as i32 - 1)..=(row as i32 + 1) {
                    for c in (col as i32 - 1)..=(col as i32 + 1) {
                        if r < 0 || r >= rows as i32 || c < 0 || c >= cols as i32 {
                            continue;
                        }
                        if board[r as usize][c as usize] == 9 {
                            count += 1;
                        }
                    }
                }
                board[row][col] = count;
            }
        }

        Minesweeper {
            board,
            rows,
            cols,
            mines,
        }
    }

    // Function to print the board
    fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                if *cell == 9 {
                    print!("9 ");
                } else {
                    print!("{} ", cell);
                }
            }
            println!();
        }
    }
}

fn main() {
    let game = Minesweeper::new(9, 9, 10);
    game.print_board();
}
