fn find_nemo(input: &str) -> String {
    println!("find nemo from string: {}", input);
    let words: Vec<&str> = input.split(" ").collect();
    for (index, word) in words.iter().enumerate() {
        if *word == "Nemo" {
            return format!("I found Nemo at {}!", index + 1);
        }
    }
    return "I can't find Nemo :(".to_string();
}

fn main() {
    let inputs: [&str; 4] = [
        "I am finding Nemo !",
        "Nemo is me",
        "I Nemo am",
        "No NEMO here",
    ];
    for input in &inputs {
        let result = find_nemo(input);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nemo() {
        let inputs = [
            "I am finding Nemo !",
            "Nemo is me",
            "I Nemo am",
            "No NEMO here",
            "Nemo is not here",
        ];

        let expected_outputs = [
            "I found Nemo at 4!",
            "I found Nemo at 1!",
            "I found Nemo at 2!",
            "I can't find Nemo :(",
            "I found Nemo at 1!",
        ];

        for (input, expected) in inputs.iter().zip(expected_outputs.iter()) {
            let result = find_nemo(input);
            assert_eq!(&result, expected);
        }
    }
}