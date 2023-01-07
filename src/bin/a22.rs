use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [Usize1; N - 1],
        B: [Usize1; N - 1]
    }
    // たどり着けないマス目から他のマス目への移動を得点に含めないようにするため、
    // 十分に大きな負の数を初期値として設定する
    let mut dp = vec![-150 * N as isize; N];
    // スタート地点を得点0にセット
    dp[0] = 0;
    for i in 0..(N - 1) {
        dp[A[i]] = std::cmp::max(dp[A[i]], dp[i] + 100);
        dp[B[i]] = std::cmp::max(dp[B[i]], dp[i] + 150);
    }
    println!("{}", dp[N - 1]);
}
