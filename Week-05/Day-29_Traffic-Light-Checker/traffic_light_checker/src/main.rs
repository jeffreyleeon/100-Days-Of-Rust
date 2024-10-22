use std::io;

enum State {
    Off,
    Red,
    Green,
    Yellow,
    FlashRed,
    FlashYellow,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let sequence: Vec<char> = input.trim().split_whitespace().map(|s| s.chars().next().unwrap()).collect();
    
    if sequence.len() > 15 {
        println!("ERROR");
        return;
    }
    
    let result = validate_sequence(&sequence);
    println!("{}", result);
}

fn validate_sequence(sequence: &[char]) -> &'static str {
    let mut state = State::Off;
    
    for &code in sequence {
        match (state, code) {
            (State::Off, 'R') => state = State::Red,
            (State::Red, 'G') => state = State::Green,
            (State::Green, 'Y') => state = State::Yellow,
            (State::Yellow, 'R') => state = State::Red,
            (State::Red, 'P') => state = State::FlashRed,
            (State::Red, 'C') => state = State::FlashYellow,
            (State::FlashRed, 'R') => state = State::Red,
            (State::FlashYellow, 'R') => state = State::Red,
            _ => return "REJECT",
        }
    }
    
    if matches!(state, State::Red | State::Yellow | State::Green) {
        "ACCEPT"
    } else {
        "REJECT"
    }
}