use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        N: usize
    }
    // 来場者数の増減を記録する配列。番兵として1つ余分に確保
    let mut B = vec![0; D + 1];
    for _ in 0..N {
        input! {
            L: Usize1,
            R: Usize1
        }
        B[L] += 1;
        B[R + 1] -= 1;
    }
    let mut answer = 0;
    for i in 0..D {
        answer = answer + B[i];
        println!("{}", answer);
    }
}