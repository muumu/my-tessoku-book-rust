use superslice::Ext;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
        L: usize,
        R: usize,
        mut X: [usize; N]
    }
    X.push(W);
    X.insert(0, 0); // X[0]: 西岸, X[N + 1]: 東岸
    let modulo: usize = 1000000007;
    let mut dp = vec![0; N + 2];
    let mut sum = vec![0; N + 2];
    dp[0] = 1;
    sum[0] = 1;
    for i in 1..=(N + 1) {
        if X[i] < L {
            dp[i] = 0;
        } else {
            let left = if X[i] > R { X.lower_bound(&(X[i] - R)) } else { 0 };
            let right = X.lower_bound(&(X[i] - L + 1)) - 1;
            dp[i] = sum[right];
            if left >= 1 {
                dp[i] += modulo - sum[left - 1];
            }
            dp[i] %= modulo;
        }
        sum[i] = (sum[i - 1] + dp[i]) % modulo;
    }
    println!("{}", dp[N + 1]);
}