use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn count_municipalities(state: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("data.json")?;
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader)?;

    let count = data.as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter(|item| {
            item["microrregiao"]["mesorregiao"]["UF"]["sigla"] == state
        })
        .count();

    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = "SP";
    match count_municipalities(state) {
        Ok(count) => println!("Number of municipalities in {}: {}", state, count),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
