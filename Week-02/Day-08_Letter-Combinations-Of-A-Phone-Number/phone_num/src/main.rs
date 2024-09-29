use itertools::Itertools;

fn letter_combinations(digits: String) -> Vec<String> {
    let digit_to_char = vec![
        vec!["a", "b", "c"],
        vec!["d", "e", "f"],
        vec!["g", "h", "i"],
        vec!["j", "k", "l"],
        vec!["m", "n", "o"],
        vec!["p", "q", "r", "s"],
        vec!["t", "u", "v"],
        vec!["w", "x", "y", "z"],
    ];

    let mut gruops: Vec<Vec<&str>> = Vec::new();
    for c in digits.chars() {
        let index = c.to_digit(10).unwrap() as usize - 2;
        if index < digit_to_char.len() {
            println!("Adding {:?}", index);
            let chars = digit_to_char[index].clone();
            gruops.push(chars);
        }
    }
    println!("Groups of chars : {:?}", gruops);
    let result = gruops.into_iter()
        .multi_cartesian_product()
        .map(|combination| combination.join(""))
        .collect();
    println!("Result : {:?}", result);
    result
}

fn main() {
    let input: String = String::from("239");
    let result = letter_combinations(input);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let input: String = String::from("23");
        let result = letter_combinations(input);
        assert_eq!(result, vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    }

    #[test]
    fn test_letter_combinations_4_digits() {
        let input: String = String::from("4569");
        let result = letter_combinations(input);
        assert_eq!(result, vec!["gjmw", "gjmx", "gjmy", "gjmz", "gjnw", "gjnx", "gjny", "gjnz", "gjow", "gjox", "gjoy", "gjoz", "gkmw", "gkmx", "gkmy", "gkmz", "gknw", "gknx", "gkny", "gknz", "gkow", "gkox", "gkoy", "gkoz", "glmw", "glmx", "glmy", "glmz", "glnw", "glnx", "glny", "glnz", "glow", "glox", "gloy", "gloz", "hjmw", "hjmx", "hjmy", "hjmz", "hjnw", "hjnx", "hjny", "hjnz", "hjow", "hjox", "hjoy", "hjoz", "hkmw", "hkmx", "hkmy", "hkmz", "hknw", "hknx", "hkny", "hknz", "hkow", "hkox", "hkoy", "hkoz", "hlmw", "hlmx", "hlmy", "hlmz", "hlnw", "hlnx", "hlny", "hlnz", "hlow", "hlox", "hloy", "hloz", "ijmw", "ijmx", "ijmy", "ijmz", "ijnw", "ijnx", "ijny", "ijnz", "ijow", "ijox", "ijoy", "ijoz", "ikmw", "ikmx", "ikmy", "ikmz", "iknw", "iknx", "ikny", "iknz", "ikow", "ikox", "ikoy", "ikoz", "ilmw", "ilmx", "ilmy", "ilmz", "ilnw", "ilnx", "ilny", "ilnz", "ilow", "ilox", "iloy", "iloz"]);
    }

    #[test]
    fn test_letter_combinations_empty() {
        let input: String = String::from("");
        let result: Vec<String> = letter_combinations(input);
        let empty: Vec<String> = Vec::new();
        assert_eq!(result, empty);
    }
}
