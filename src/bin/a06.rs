use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N]
    }
    // Aを1-originに変更（A[i] = i日目の来場者数, A[0] = 0）
    let A = [vec![0], A].concat();
    // Sも1-originに統一
    let mut S = vec![0; N + 1];
    for i in 1..=N {
        S[i] = S[i - 1] + A[i];
    }
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize
        }
        let answer = S[R] - S[L - 1];
        println!("{}", answer);
    }
}