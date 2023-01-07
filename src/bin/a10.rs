use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        D: usize,
    }
    // 部屋の号室は0-originとする
    let mut lmax = vec![0; N];
    let mut rmax = vec![0; N];
    lmax[0] = A[0];
    for i in 1..N {
        lmax[i] = std::cmp::max(lmax[i - 1], A[i]);
    }
    rmax[N - 1] = A[N - 1];
    for i in (0..(N - 1)).rev() {
        rmax[i] = std::cmp::max(rmax[i + 1], A[i]);
    }
    for _ in 0..D {
        input! {
            L: Usize1,
            R: Usize1
        }
        let answer = std::cmp::max(lmax[L - 1], rmax[R + 1]);
        println!("{}", answer);
    }
}
