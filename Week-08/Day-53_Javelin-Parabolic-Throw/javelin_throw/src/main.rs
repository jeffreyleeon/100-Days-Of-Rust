fn javelin_throw(velocity: f64, angle: f64) {
    let g = 9.81;
    let angle = angle.to_radians();
    let time = (2.0 * velocity * angle.sin()) / g;
    let x_distance = velocity * time * angle.cos();
    let y_distance = velocity * time * 0.5 * angle.sin() - 0.5 * g * (time * 0.5).powi(2);
    println!("Ymax={}m, Xmax={}m", y_distance as i32, x_distance as i32);
}

fn main() {
    javelin_throw(36.7, 45.0);
}
