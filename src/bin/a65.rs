use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [Usize1; N - 1]
    }
    let mut G = vec![vec![]; N];
    for i in 0..(N - 1) {
        G[A[i]].push(i + 1);
    }
    // dp[i]: 社員iの部下の数、A[i - 1]: 社員iの直属の上司
    // ただし社員番号は0-origin
    let mut dp = vec![0; N];
    dp[N - 1] = 0;
    for i in (0..(N - 1)).rev() {
        for &n in &G[i] {
            dp[i] += dp[n] + 1;
        }
    }
    let ans: Vec<String> = dp.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
