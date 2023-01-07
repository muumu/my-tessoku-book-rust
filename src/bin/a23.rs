use proconio::input;
use std::cmp::min;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [[usize; N]; M]
    }
    let mut bitA = vec![0; M];
    for i in 0..M {
        for j in 0..N {
            bitA[i] |= A[i][j] << j;
        }
    }
    let b = 1 << N;
    let mut dp = vec![vec![101; b]; M + 1];
    dp[0][0] = 0;
    for i in 0..M {
        for j in 0..b {
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
            dp[i + 1][j | bitA[i]] = min(dp[i + 1][j | bitA[i]], dp[i][j] + 1);
        }
    }
    if dp[M][b - 1] > 100 {
        println!("-1");
    } else {
        println!("{}", dp[M][b - 1]);
    }
}
