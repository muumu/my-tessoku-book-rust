use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
        wv: [(usize, usize); N]
    }
    // dp[i + 1][j]: 0～i番目までの品物で重さをj以下にするときの最大値
    let mut dp = vec![vec![0; W + 1]; N + 1];
    for j in 0..=W {
        dp[0][j] = 0;
    }
    for i in 0..N {
        for j in 0..=W {
            if j < wv[i].0 {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = std::cmp::max(dp[i][j], 
                    dp[i][j - wv[i].0] + wv[i].1);
            }
        }
    }
    println!("{}", dp[N][W]);
}
