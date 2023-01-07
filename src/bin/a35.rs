use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut B = vec![vec![0; N]; N];
    B[N - 1] = A;
    // 最下段から遡ると、偶数段目はスコアを最大化する選択をし、
    // 奇数段目はスコアを最小化する選択をする
    for i in (0..(N - 1)).rev() {
        for j in 0..(i + 1) {
            if i % 2 == 0 {
                B[i][j] = std::cmp::max(B[i + 1][j], B[i + 1][j + 1]);
            } else {
                B[i][j] = std::cmp::min(B[i + 1][j], B[i + 1][j + 1]);
            }
        }
    }
    println!("{}", B[0][0]);
}
