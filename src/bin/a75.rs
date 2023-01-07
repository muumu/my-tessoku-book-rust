use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut TD: [(usize, usize); N]
    }
    // 締切の早い順に問題をソート
    TD.sort_by(|a, b| a.1.cmp(&b.1));
    // println!("{:?}", TD);
    let maxD = TD[N - 1].1; // 締切の最大値（許容される経過時間の最大値）
    // dp[i][j]: i番目の問題を解かないか解くかしてjだけ時間が経過したときの最大可能正解数
    let mut dp = vec![vec![0; maxD + 1]; N + 1];
    for i in 1..=N {
        for j in 0..=maxD {
            let T = TD[i - 1].0; // TDは0-originなのでiから1を引く
            let D = TD[i - 1].1;
            if j < T || j > D {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j - T] + 1, dp[i - 1][j]);
            }
        }
    }
    println!("{}", dp[N].iter().max().unwrap());
}
