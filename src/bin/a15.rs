use itertools::Itertools;
use proconio::input;
use itertools::sorted;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let C = sorted(A.clone()).unique().collect_vec();
    let mut B = vec![String::new(); N];
    for i in 0..N {
        // Cは0-originなためCのインデックスに1を足してB[i]>=1を満たすようにする
        B[i] = (C.binary_search(&A[i]).unwrap() + 1).to_string();
    }
    println!("{}", B.join(" "));
}
