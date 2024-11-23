fn tic_tac_toe(board: [[char; 3]; 3]) -> &'static str {
    // Check rows and columns for a winner
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            if board[i][0] == 'X' {
                return "Player 1 wins";
            } else if board[i][0] == 'O' {
                return "Player 2 wins";
            }
        }
        
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            if board[0][i] == 'X' {
                return "Player 1 wins";
            } else if board[0][i] == 'O' {
                return "Player 2 wins";
            }
        }
    }

    // Check diagonals for a winner
    if (board[0][0] == board[1][1] && board[1][1] == board[2][2]) || 
       (board[0][2] == board[1][1] && board[1][1] == board[2][0]) {
        if board[1][1] == 'X' {
            return "Player 1 wins";
        } else if board[1][1] == 'O' {
            return "Player 2 wins";
        }
    }

    // Check for empty spaces to determine if it's a tie
    for row in &board {
        for &cell in row {
            if cell == '#' {
                return "Game is still ongoing";
            }
        }
    }

    // If no winners and no empty spaces, it's a tie
    "It's a Tie"
}

fn main() {
    let game_board = [
        ['X', 'O', 'O'],
        ['O', 'X', 'O'],
        ['O', '#', 'X'],
    ];

    println!("{}", tic_tac_toe(game_board)); // Output: Player 1 wins
}