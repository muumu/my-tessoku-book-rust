use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N]
    }
    for a in A {
        if a == X {
            println!("Yes");
            return;
        }
    }
    println!("No");
}