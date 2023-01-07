use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
        K: usize,
        A: [usize; N]
    }
    let mut low = 0;
    // 均等にK+1分割したときの幅より大きなスコアは不可能
    let mut high = L / (K + 1) + 1;
    // lowとhighが一致したときにはlowが答えとなっている
    while low < high {
        // lowはokであることがわかっているため、
        // midはlowとmidの平均を四捨五入してhighに寄せる
        // たとえばlowが2でhighが3のとき、midを3にしたい
        let mid = (low + high + 1) / 2;
        let mut count = 0;
        let mut prev = 0;
        let mut ok = false;
        for i in 0..N {
            if A[i] - prev >= mid {
                if count + N - i < K {
                    continue;
                }
                count += 1;
                prev = A[i];
            }
            if count == K && L - prev >= mid {
                ok = true;
            }
        }
        if ok {
            low = mid; // midが答えかもしれないのでlowにはmidをそのまま入れる
        } else {
            high = mid - 1; // 答えはmid未満なので1を引く
        }
    }
    println!("{}", low);
}