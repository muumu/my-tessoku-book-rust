use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize
    }
    if K >= (N - 2) * 2 && K % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
