fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut count = [0; 26];

    for (sc, tc) in s.chars().zip(t.chars()) {
        count[(sc as usize) - ('a' as usize)] += 1;
        count[(tc as usize) - ('a' as usize)] -= 1;
    }

    count.iter().all(|&x| x == 0)
}

fn main() {
    let s: String = String::from("anagram");
    let t: String = String::from("nagaram");
    let result = is_anagram(s, t);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram(String::from("anagram"), String::from("nagaram")), true);
        assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
        assert_eq!(is_anagram(String::from("listen"), String::from("silent")), true);
        assert_eq!(is_anagram(String::from("evil"), String::from("vile")), true);
        assert_eq!(is_anagram(String::from("fluster"), String::from("restful")), true);
        assert_eq!(is_anagram(String::from("binary"), String::from("brainy")), true);
        assert_eq!(is_anagram(String::from("adobe"), String::from("abode")), true);
        assert_eq!(is_anagram(String::from("night"), String::from("thing")), true);
        assert_eq!(is_anagram(String::from("dusty"), String::from("study")), true);
        assert_eq!(is_anagram(String::from("inch"), String::from("chin")), true);
        assert_eq!(is_anagram(String::from("brag"), String::from("grab")), true);
        assert_eq!(is_anagram(String::from("cat"), String::from("act")), true);
        assert_eq!(is_anagram(String::from("save"), String::from("vase")), true);
    }
}