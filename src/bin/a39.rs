use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut LR: [(usize, usize); N]
    }
    LR.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
    let mut ans = 0;
    let mut current = 0;
    for film in LR {
        if film.0 >= current {
            ans += 1;
            current = film.1;
        }
    }
    println!("{}", ans);
}
