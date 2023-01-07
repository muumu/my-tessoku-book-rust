use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize
    }
    let d = 1000000007;
    let mut i = 0;
    let mut p = a;
    let mut ans = 1;
    while (b >> i) > 0 {
        if b & (1 << i) > 0 {
            ans = (ans * p) % d;
        }
        p = (p * p) % d;
        i += 1;
    }
    println!("{}", ans);
}
