use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        mut B: [usize; N]
    }
    A.sort();
    B.sort();
    B.reverse();
    let mut ans = 0;
    for i in 0..N {
        ans += A[i] * B[i];
    }
    println!("{}", ans);
}