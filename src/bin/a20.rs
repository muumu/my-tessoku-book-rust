use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars
    }
    // S, Tは0-origin、dpは1-origin
    let mut dp = vec![vec![0; T.len() + 1]; S.len() + 1];
    for i in 0..S.len() {
        for j in 0..T.len() {
            if S[i] == T[j] {
                dp[i + 1][j + 1] = max(max(dp[i + 1][j], dp[i][j + 1]), dp[i][j] + 1);
            } else {
                dp[i + 1][j + 1] = max(dp[i + 1][j], dp[i][j + 1])
            }
        }
    }
    println!("{}", dp[S.len()][T.len()]);
}
