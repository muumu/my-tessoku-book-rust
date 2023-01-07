use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        AB: [(usize, usize); N]
    }
    let mut ans = 0;
    for minA in 1..=100 {
        for minB in 1..=100 {
            let mut count = 0;
            for &(A, B) in &AB {
                if A >= minA && A <= minA + K && B >= minB && B <= minB + K {
                    count += 1;
                }
            }
            ans = std::cmp::max(count, ans);
        }
    }
    println!("{}", ans);
}
