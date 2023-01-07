use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }
    let mut n: isize = 0;
    for _ in 0..N {
        input! {
            T: char,
            A: isize
        }
        if T == '+' {
            n += A;
        } else if T == '-' {
            n -= A;
            if n < 0 {
                n += 10000;
            }
        } else {
            n *= A;
        }
        n %= 10000;
        println!("{}", n);
    }
}
