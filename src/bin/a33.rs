use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let xorA = A.into_iter().fold(0, |a, b| a ^ b);
    let ans = if xorA > 0 { "First" } else { "Second" };
    println!("{}", ans);
}
