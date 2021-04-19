use proconio::input;

fn bingo () {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n]
    }

    let mut check = vec![vec![false, false, false], vec![false, false, false], vec![false, false, false]];

    for n_i in 0..n {
        for row_i in 0..3 {
            for col_i in 0..3 {
                if a[row_i][col_i] == b[n_i] {
                    check[row_i][col_i] = true
                }
            }
        }
    }

    let mut is_bingo = false;

    for row_i in 0..3 {
        if check[row_i][0] && check[row_i][1] && check[row_i][2] {
            is_bingo = true;
        }
    }

    for col_i in 0..3 {
        if check[0][col_i] && check[1][col_i] && check[2][col_i] {
            is_bingo = true;
        }
    }

    if check[0][0] && check[1][1] && check[2][2] {
        is_bingo = true;
    }

    if check[0][2] && check[1][1] && check[2][0] {
        is_bingo = true;
    }

    println!("{}", if is_bingo { "Yes" } else { "No" });
}

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

    // 第1回PASTのC問題 Bingoを解く
    bingo();
}
