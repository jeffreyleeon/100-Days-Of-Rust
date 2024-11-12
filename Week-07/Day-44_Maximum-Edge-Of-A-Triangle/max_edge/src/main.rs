fn next_edge(side1: i32, side2: i32) -> i32 {
    (side1 + side2) - 1
}

fn main() {
    println!("{}", next_edge(8, 10));
    println!("{}", next_edge(5, 7));
    println!("{}", next_edge(9, 2));
}
