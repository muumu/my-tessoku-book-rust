use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        C: [Chars; H]
    }
    let mut dp: Vec<Vec<u64>> = vec![vec![0; W]; H];
    if C[0][0] == '.' {
        dp[0][0] = 1;
    }
    for i in 0..H {
        for j in 0..W {
            if i == 0 {
                if j > 0 && C[i][j] == '.' {
                    dp[i][j] = dp[i][j - 1];
                }
            } else {
                if j == 0 {
                    if C[i][j] == '.' {
                        dp[i][j] = dp[i - 1][j];
                    }
                } else {
                    if C[i][j] == '.' { 
                        dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                    }
                }
            }
        }
    }
    println!("{}", dp[H - 1][W - 1]);
}
