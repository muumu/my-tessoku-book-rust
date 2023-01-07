use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
        D: [usize; N],
    }
    let mut Sab = vec![0; N * N];
    let mut Scd = vec![0; N * N];
    for i in 0..N {
        for j in 0..N {
            Sab[N * i + j] = A[i] + B[j];
            Scd[N * i + j] = C[i] + D[j];
        }
    }
    Scd.sort();
    for &s in &Sab {
        if K > s && Scd.binary_search(&(K - s)).is_ok() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
