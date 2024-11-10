fn area_of_triangle(base: f64, height: f64) -> f64 {
    0.5 * base * height
}

fn main() {
    let base = 10.0;
    let height = 20.0;
    let area = area_of_triangle(base, height);
    println!("Area of triangle with base {} and height {} is {}", base, height, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_of_triangle() {
        assert_eq!(area_of_triangle(10.0, 20.0), 100.0);
        assert_eq!(area_of_triangle(5.0, 10.0), 25.0);
        assert_eq!(area_of_triangle(15.0, 30.0), 225.0);
    }
}
