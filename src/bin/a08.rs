use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        X: [[isize; W]; H],
        Q: usize
    }
    let mut S = vec![vec![0isize; W + 1]; H + 1];
    for i in 1..=H {
        for j in 1..=W {
            // Xだけ0-originなのでインデックスから1を引く
            S[i][j] = S[i][j - 1] + X[i - 1][j - 1];
        }
    }
    for j in 1..=W {
        for i in 1..=H {
            S[i][j] += S[i - 1][j];
        }
    }
    for _ in 0..Q {
        input! {
            A: usize,
            B: usize,
            C: usize,
            D: usize
        }
        let answer = S[C][D] - S[A - 1][D] - S[C][B - 1] + S[A - 1][B - 1];
        println!("{}", answer);
    }
}