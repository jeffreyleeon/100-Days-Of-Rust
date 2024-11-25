fn switch_gravity_on(mut a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..a[0].len() {
        let mut count = 0;
        for j in 0..a.len() {
            if a[j][i] == '#' {
                count += 1;
            }
        }
        let num_of_rows = a.len();
        for j in 0..num_of_rows {
            if count > 0 {
                a[num_of_rows - 1 - j][i] = '#';
                count -= 1;
            } else {
                a[num_of_rows - 1 - j][i] = '.';
            }
        }
    }
    a
}

fn main() {
    let a = vec![
        vec!['.', '#', '#', '.'],
        vec!['#', '.', '.', '#'],
        vec!['.', '.', '.', '#'],
        vec!['.', '.', '#', '.'],
    ];
    let result = switch_gravity_on(a);
    println!("{:?}", result);
}
