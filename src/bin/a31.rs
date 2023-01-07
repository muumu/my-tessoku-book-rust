use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }
    let ans = N / 3 + N / 5 - N / 15;
    println!("{}", ans);
}
