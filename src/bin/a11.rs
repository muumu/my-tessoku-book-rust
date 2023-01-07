use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N] // Aの要素は小さい順に整列済み
    }
    let mut low = 0;
    let mut high = N - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if A[mid] == X {
            println!("{}", mid + 1);
            return;
        } else if A[mid] > X {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
}
