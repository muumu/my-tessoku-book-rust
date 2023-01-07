use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N]
    }
    // dp[i][j]: 0～i-1番目までのカードの中からいくつか
    // 選んだ値の合計がjになる方法が存在するかどうかの真偽値
    let mut dp = vec![vec![false; 10001]; N];
    // dp[0]でtrueになるのは値が0のときとA[0]のときのみ
    dp[0][0] = true;
    dp[0][A[0]] = true;
    for i in 1..N {
        for j in 0..=S {
            if dp[i - 1][j] || (j >= A[i] && dp[i - 1][j - A[i]]) {
                dp[i][j] = true;
            }
        }
    }
    if dp[N - 1][S] {
        println!("Yes");
    } else {
        println!("No");
    }
}