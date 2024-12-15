fn main() {
    println!("Hello, world!");
    let mut current_state = "CLOSED";
    let input_commands = vec![
        "button_clicked",
        "cycle_complete",
        "button_clicked",
        "button_clicked",
        "button_clicked",
        "button_clicked",
        "button_clicked",
        "cycle_complete",
    ];
    println!("Current state: {}", current_state);
    for command in input_commands {
        println!("> Command: {}", command);
        match command {
            "button_clicked" => {
                current_state = match current_state {
                    "CLOSED" => "OPENING",
                    "OPENING" => "STOPPED_WHILE_OPENING",
                    "CLOSING" => "STOPPED_WHILE_CLOSING",
                    "OPEN" => "CLOSING",
                    "STOPPED_WHILE_CLOSING" => "OPENING",
                    "STOPPED_WHILE_OPENING" => "CLOSING",
                    _ => "UNKNOWN",
                };
            }
            "cycle_complete" => {
                current_state = match current_state {
                    "OPENING" => "OPEN",
                    "CLOSING" => "CLOSED",
                    _ => "UNKNOWN",
                };
            }
            _ => {
                println!("Unknown command");
            }
        }
        println!("Current state: {}", current_state);
    }
}
