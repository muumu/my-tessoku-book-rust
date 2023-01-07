use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize
    }
    // i個の石がある局面において先手必勝の場合0、後手必勝の場合1とする
    let mut dp = vec![0; N + 1];
    for i in 0..=N {
        if i < A {
            dp[i] = 1;
        } else if i < A + B {
            dp[i] = 0;
        } else {
            if dp[i - A] == 1 || dp[i - B] == 1 {
                dp[i] = 0;
            } else {
                dp[i] = 1;
            }
        }
    }
    let ans = if dp[N] == 0 { "First" } else { "Second" };
    println!("{}", ans);
}
