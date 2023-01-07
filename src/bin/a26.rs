use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize
    }
    for _ in 0..Q {
        input! {
            X: usize
        }
        let mut i = 2;
        let mut is_prime = true;
        while i * i <= X {
            if X % i == 0 {
                is_prime = false;
            }
            i += 1;
        }
        if is_prime {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}