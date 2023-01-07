use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        B: usize,
        A: [usize; N],
        C: [usize; M]
    }
    // (A_1+B+C_1+...+A_1+B+C_M)+...+(A_N+B+C_1+...+A_N+B+C_M)
    // = M(A_1+A_2+...+A_N)+N(C_1+C_2+...+C_M)+NMB
    let sumA: usize = A.iter().sum();
    let sumC: usize = C.iter().sum();
    let ans = M * sumA + N * M * B + N * sumC;
    println!("{}", ans);
}
