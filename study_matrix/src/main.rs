fn print_matrix() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    for row in matrix.iter() {
        for col in row.iter() {
            println!("{}", col)
        }
    }
}

fn main() {
    println!("--- print_matrix() ---");
    print_matrix();
    println!("--- END ---");
}
