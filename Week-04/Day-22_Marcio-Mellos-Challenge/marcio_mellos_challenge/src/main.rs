fn km2_to_football_fields(km2: f64) -> u64 {
    // Convert km² to m²
    let area_m2 = km2 * 1_000_000.0;

    const FIELD_LENGTH: f64 = 105.0; // meters
    const FIELD_WIDTH: f64 = 68.0; // meters
    let field_area_m2 = FIELD_LENGTH * FIELD_WIDTH;

    let fields = (area_m2 / field_area_m2).round() as u64;
    fields
}

fn main() {
    let km2: f64 = 1.034; //km2
    let field: u64 = km2_to_football_fields(km2);
    println!("{:.3} km² is equal to {} fields", km2, field);
}