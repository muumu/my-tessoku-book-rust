use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N - 1],
        B: [usize; N - 2]
    }
    let mut dp = vec![0; N];
    dp[0] = 0;
    dp[1] = A[0];
    for i in 2..N {
        dp[i] = std::cmp::min(dp[i - 1] + A[i - 1], dp[i - 2] + B[i - 2]);
    }
    let mut answer = vec![N];
    let mut i = N - 1;
    while i >= 1 {
        if dp[i] == dp[i - 1] + A[i - 1] {
            answer.push(i);
            i -= 1;
        } else {
            answer.push(i - 1);
            i -= 2;
        }
    }
    let answer = answer.into_iter().rev().map(|x| x.to_string()).collect_vec();
    let K = answer.len();
    println!("{}", K);
    println!("{}", answer.join(" "));
}