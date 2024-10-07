/*
Strategy to win:
    If you're not on a multiple of 4, you can always remove stones to leave your opponent with a multiple of 4.
    For example:
        If there are 5 stones, remove 1 to leave 4.
        If there are 6 stones, remove 2 to leave 4.
        If there are 7 stones, remove 3 to leave 4.
Continuing the game:
    After reducing to a multiple of 4, your opponent must take 1, 2, or 3 stones.
    No matter what they do, you can again reduce the pile to a multiple of 4.
    This cycle continues until your opponent is left with exactly 4 stones.
Endgame:
    When your opponent has 4 stones, they must take 1, 2, or 3, leaving you with 3, 2, or 1 stone respectively.
    You can then take all remaining stones and win.
*/
fn can_win(input: u8) -> bool {
    input % 4 != 0
}

fn main() {
    let input: u8 = 10;
    println!("Can win: {}", can_win(input));
}
