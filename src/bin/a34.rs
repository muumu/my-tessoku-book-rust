use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        A: [usize; N]
    }
    let mut grundy = vec![0; 100001];
    for i in 0..grundy.len() {
        if i < X {
            grundy[i] = 0;
        } else if i < Y {
            grundy[i] = if grundy[i - X] == 0 { 1 } else { 0 }
        } else {
            // 0, 0または0, 2の場合（1ビット目が両方とも0の場合）
            if (grundy[i - X] | grundy[i - Y]) & 1 == 0 {
                grundy[i] = 1;
            } else if grundy[i - X] + grundy[i - Y] == 1 {
                grundy[i] = 2;
            } else {
                grundy[i] = 0;
            }
        }
    }
    let xor = A.into_iter().fold(0, |a, b| a ^ grundy[b]);
    let ans = if xor > 0 { "First" } else { "Second" };
    println!("{}", ans);
}
