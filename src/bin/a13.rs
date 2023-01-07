use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }
    let mut p = 1;
    let mut answer = 0;
    for i in 0..(N - 1) {
        while p < N && A[p] - A[i] <= K {
            p += 1;
        }
        answer += (p - 1) - i;
    }
    println!("{}", answer);
}
