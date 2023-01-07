use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize
    }
    if A >= B {
        println!("{}", gcd(A, B));
    } else {
        println!("{}", gcd(B, A));
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let r = a % b;
    if r == 0 {
        return b;
    } else {
        return gcd(b, r);
    }
}
