fn christmas_tree(n: i32) -> String {
    let mut tree = String::new();
    let mut spaces = n - 1;
    let mut stars = 1;
    for _ in 0..n {
        for _ in 0..spaces {
            tree.push(' ');
        }
        for _ in 0..stars {
            tree.push('*');
        }
        tree.push('\n');
        spaces -= 1;
        stars += 2;
    }
    tree
}

fn main() {
    println!("Christmas Tree 0");
    println!("{}", christmas_tree(0));
    println!("Christmas Tree 1");
    println!("{}", christmas_tree(1));
    println!("Christmas Tree 2");
    println!("{}", christmas_tree(2));
    println!("Christmas Tree 3");
    println!("{}", christmas_tree(3));
    println!("Christmas Tree 4");
    println!("{}", christmas_tree(4));
    println!("Christmas Tree 5");
    println!("{}", christmas_tree(5));
}
