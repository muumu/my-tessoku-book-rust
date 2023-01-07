use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [[usize; N]; N]
    }
    let mut I = vec![0; N];
    let mut J = vec![0; N];
    for i in 0..N {
        for j in 0..N {
            if P[i][j] != 0 {
                let k = P[i][j] - 1; // 整数を0-originにする
                I[k] = i;
                J[k] = j;
            }
        }
    }
    let mut cnt = 0;
    for k in 0..N {
        for l in k..N {
            if I[k] > I[l] {
                cnt += 1;
            }
            if J[k] > J[l] {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
