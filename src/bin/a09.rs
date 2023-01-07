use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize
    }
    // 1-originと番兵の分の2マスを余分に確保しておく
    let mut diff = vec![vec![0; W + 2]; H + 2];
    // A, B, C, Dは1-originとする
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
            C: usize,
            D: usize
        }
        diff[A][B] += 1;
        diff[A][D + 1] -= 1;
        diff[C + 1][B] -= 1;
        diff[C + 1][D + 1] += 1;
    }
    let mut Z = vec![vec![0; W + 1]; H + 1];
    for i in 1..=H {
        for j in 1..=W {
            Z[i][j] = Z[i][j - 1] + diff[i][j];
        }
    }
    for i in 1..=H {
        for j in 1..=W {
            Z[i][j] += Z[i - 1][j];
        }
    }
    for i in 1..=H {
        let line: Vec<String> = Z[i][1..=W].iter().map(|&x| x.to_string()).collect();
        println!("{}", line.join(" "));
    }
}
