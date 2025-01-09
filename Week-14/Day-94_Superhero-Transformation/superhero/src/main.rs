use std::io;

fn is_transformable(s: &str, t: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    if s.len() != t.len() {
        return false;
    }
    
    for (c1, c2) in s.chars().zip(t.chars()) {
        let is_vowel1 = vowels.contains(&c1);
        let is_vowel2 = vowels.contains(&c2);
        
        if is_vowel1 != is_vowel2 {
            return false;
        }
    }
    
    true
}

fn main() {
    let mut s = String::new();
    let mut t = String::new();

    println!("Enter the first superhero name:");
    io::stdin().read_line(&mut s).unwrap();
    println!("Enter the second superhero name:");
    io::stdin().read_line(&mut t).unwrap();

    // Trim whitespace and process input
    let s = s.trim();
    let t = t.trim();

    if is_transformable(s, t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
