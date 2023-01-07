use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        r: usize
    }
    if n == r {
        println!("{}", 1);
        return;
    }
    let d = 1000000007; // 素数
    // nCr = n*(n-1)*...*(n-r+1)/r!で計算する
    let mut a = 1;
    for i in (n - r + 1)..=n {
        a = (a * i) % d;
    }
    let mut b = 1;
    for i in 1..=r {
        b = (b * i) % d;
    }
    // フェルマーの小定理より
    let ans = (a * pow(b, d - 2, d)) % d;
    println!("{}", ans);
}

fn pow(a: usize, b: usize, d: usize) -> usize {
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
    return ans;
}