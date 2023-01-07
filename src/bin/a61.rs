use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    // 頂点番号は1-originとする
    let mut graph = vec![vec![]; N + 1];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize
        }
        graph[A].push(B);
        graph[B].push(A);
    }
    for i in 1..=N {
        let s = graph[i].iter().join(", ");
        println!("{}: {{{}}}", i, s);
    }
}
