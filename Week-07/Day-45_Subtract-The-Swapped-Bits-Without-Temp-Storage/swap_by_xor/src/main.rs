fn subtract_by_xor(a: i32, b: i32) -> i32 {
    let c = a ^ a ^ b;
    let d = b ^ b ^ a;
    c - d
}

fn main() {
    println!("{}", subtract_by_xor(5, 10));
    println!("{}", subtract_by_xor(10, 41));
    println!("{}", subtract_by_xor(69, 420));
    println!("{}", subtract_by_xor(12345, 890412));
    println!("{}", subtract_by_xor(2, 1));
}
