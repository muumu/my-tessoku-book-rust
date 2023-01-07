use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: char
        }
        let t = if B == 'E' { L - A } else { A };
        ans = std::cmp::max(t, ans);
    }
    println!("{}", ans);
}
