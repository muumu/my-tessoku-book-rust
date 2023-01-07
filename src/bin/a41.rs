use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars
    }
    for i in 0..(N - 2) {
        if S[i] == S[i + 1] && S[i + 1] == S[i + 2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
