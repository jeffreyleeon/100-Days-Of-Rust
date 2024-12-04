use rand::Rng;

fn roll_dice(num_of_dice: u32, side_of_dice: u32) {
    let mut total = 0;
    let mut rolls = Vec::new();
    for _ in 0..num_of_dice {
        let roll = rand::random::<u32>() % side_of_dice + 1;
        rolls.push(roll.to_string());
        total += roll;
    }
    println!("{total}: {rolls}", total = total, rolls = rolls.join(" "));
}

fn main() {
    roll_dice(3, 6);
    roll_dice(2, 4);
    roll_dice(1, 8);
    roll_dice(4, 12);
    roll_dice(5, 20);
    roll_dice(6, 100);
}
