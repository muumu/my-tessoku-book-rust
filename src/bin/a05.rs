use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize
    }
    let mut count = 0;
    for r in 1..=N {
        for b in 1..=N {
            if r + b < K && r + b + N >= K {
                count += 1;
            }
        }
    }
    println!("{}", count);
}