use std::collections::VecDeque;
use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N]
    }
    let powN = 2usize.pow(N as u32);
    // 頂点: ランプのオンオフの列をビット表現したもの
    // 辺: M種類の操作による状態推移
    let mut G = vec![vec![]; powN];
    for _ in 0..M {
        input! {
            X: Usize1,
            Y: Usize1,
            Z: Usize1
        }
        for state in 0..powN {
            let bits = (1 << X) | (1 << Y) | (1 << Z);
            G[state].push(state ^ bits);
        }
    }
    let mut start = 0;
    for i in 0..N {
        start |= A[i] << i;
    }
    let mut queue = VecDeque::new();
    let mut dist = vec![-1; powN];
    dist[start] = 0;
    queue.push_back(start);
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        for &to in &G[pos] {
            if dist[to] == -1 {
                dist[to] = dist[pos] + 1;
                queue.push_back(to);
            }
        }
    }
    println!("{}", dist[powN - 1]);
}
