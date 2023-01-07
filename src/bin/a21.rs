use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        PA: [(Usize1, usize); N] // 番号は0-originとする
    }
    // dpには区間[0, N-1]の状態から獲得できる最大得点を記録する
    let mut dp = vec![vec![0; N]; N];
    // 残り1個から獲得可能な得点は0なため、i = N - 1, i = jのときは初期値0のままとする
    for i in (0..(N - 1)).rev() {
        for j in (i + 1)..N {
            let a = if PA[i].0 > i && PA[i].0 <= j { PA[i].1 } else { 0 };
            let b = if PA[j].0 >= i && PA[j].0 < j { PA[j].1 } else { 0 };
            dp[i][j] = std::cmp::max(dp[i + 1][j] + a, dp[i][j - 1] + b);
        }
    }
    println!("{}", dp[0][N - 1]);
}