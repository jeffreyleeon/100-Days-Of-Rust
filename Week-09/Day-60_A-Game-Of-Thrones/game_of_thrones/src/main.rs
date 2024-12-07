fn game_of_three(input: u32) {
    let mut num = input;
    while num != 1 {
        if num % 3 == 0 {
            println!("{} 0", num);
            num /= 3;
        } else if (num + 1) % 3 == 0 {
            println!("{} 1", num);
            num = (num + 1) / 3;
        } else {
            println!("{} -1", num);
            num = (num - 1) / 3;
        }
    }
    println!("1");
}

fn main() {
    let num = 100;
    game_of_three(num);
}
