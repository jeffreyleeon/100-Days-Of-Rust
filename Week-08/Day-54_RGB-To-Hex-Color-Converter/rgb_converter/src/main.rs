use regex::Regex;

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    let r = r.clamp(0, 255);
    let g = g.clamp(0, 255);
    let b = b.clamp(0, 255);
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn extract_rgb(rgb_str: &str) -> Option<(u8, u8, u8)> {
    // Define a regex pattern to capture RGB values
    let re = Regex::new(r"rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    
    // Use regex to find captures in the input string
    if let Some(caps) = re.captures(rgb_str) {
        // Parse captured strings into u8 integers
        let r = caps[1].parse::<u8>().ok()?;
        let g = caps[2].parse::<u8>().ok()?;
        let b = caps[3].parse::<u8>().ok()?;
        
        // Return the RGB values as a tuple
        Some((r, g, b))
    } else {
        None // Return None if the input format is incorrect
    }
}

fn main() {
    let rgb_string = "rgb(0, 128, 192)";
    
    match extract_rgb(rgb_string) {
        Some((r, g, b)) => println!("{}", rgb_to_hex(r, g, b)),
        None => println!("Invalid RGB string format"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(0, 128, 192), "#0080C0");
        assert_eq!(rgb_to_hex(45, 255, 192), "#2DFFC0");
        assert_eq!(rgb_to_hex(255, 255, 255), "#FFFFFF");
        assert_eq!(rgb_to_hex(255, 0, 0), "#FF0000");
        assert_eq!(rgb_to_hex(0, 255, 0), "#00FF00");
        assert_eq!(rgb_to_hex(0, 0, 255), "#0000FF");
    }

    #[test]
    fn test_extract_rgb() {
        assert_eq!(extract_rgb("rgb(0, 128, 192)"), Some((0, 128, 192)));
        assert_eq!(extract_rgb("rgb(255, 255, 255)"), Some((255, 255, 255)));
        assert_eq!(extract_rgb("rgb(255, 0, 0)"), Some((255, 0, 0)));
        assert_eq!(extract_rgb("rgb(0, 255, 0)"), Some((0, 255, 0)));
        assert_eq!(extract_rgb("rgb(0, 0, 255)"), Some((0, 0, 255)));
        assert_eq!(extract_rgb("rgb(0, 0, 0)"), Some((0, 0, 0)));
        assert_eq!(extract_rgb("rgb(255, 255, 256)"), None);
        assert_eq!(extract_rgb("rgb(255, 255, 255, 255)"), None);
        assert_eq!(extract_rgb("rgb(255, 255)"), None);
        assert_eq!(extract_rgb("rgb(255, 255, 255"), None);
    }
}