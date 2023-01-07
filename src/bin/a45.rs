use proconio::input;
use proconio::marker::Chars;

// Wを0、Bを1、Rを2とすると、6種類の操作が以下の数式で表現できることに着目
// つまり、mod 3の世界では各操作を施してもカードの合計値が不変となる
// WW -> W: 0 + 0 ≡ 0 (mod 3)
// WB -> B: 0 + 1 ≡ 1 (mod 3)
// BB -> R: 1 + 1 ≡ 2 (mod 3)
// WR -> R: 0 + 2 ≡ 2 (mod 3)
// RR -> B: 2 + 2 ≡ 1 (mod 3)
// BR -> W: 1 + 2 ≡ 0 (mod 3)

#[allow(non_snake_case)]
fn main() {
    input! {
        _: usize,
        C: char,
        A: Chars
    }
    let C = color_to_num(C);
    let mut r = 0;
    for c in A {
        r = (r + color_to_num(c)) % 3;
    }
    if r == C {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn color_to_num(c: char) -> usize {
    if c == 'W' {
        0
    } else if c == 'B' {
        1
    } else {
        2
    }
}
