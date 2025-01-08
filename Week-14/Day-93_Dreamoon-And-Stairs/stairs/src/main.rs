fn climb_moves(n: i32, m: i32) -> i32 {
    let mut moves = (n + 1) / 2;  // Minimum number of moves
    
    while moves <= n {
        if moves % m == 0 {
            return moves;
        }
        moves += 1;
    }
    
    -1  // No solution found
}

fn main() {
    println!("{}", climb_moves(10, 2));
    println!("{}", climb_moves(3, 5));
    println!("{}", climb_moves(13, 2));
    println!("{}", climb_moves(6, 4));
    println!("{}", climb_moves(7, 4));
    println!("{}", climb_moves(8, 4));
    println!("{}", climb_moves(9, 4));
    println!("{}", climb_moves(5, 3));
    println!("{}", climb_moves(6, 3));
    println!("{}", climb_moves(7, 3));
}
