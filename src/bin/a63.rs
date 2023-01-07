use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize
    }
    // 頂点番号は0-originとする
    let mut graph = vec![vec![]; N];
    for _ in 0..M {
        input! {
            A: Usize1,
            B: Usize1
        }
        graph[A].push(B);
        graph[B].push(A);
    }
    let mut dist = vec![-1; N];
    let mut queue: VecDeque<usize> = VecDeque::new();
    dist[0] = 0;
    queue.push_front(0);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for &next_v in &graph[v] {
            if dist[next_v] == -1 {
                dist[next_v] = dist[v] + 1;
                queue.push_back(next_v);
            }
        }
    }
    for d in dist {
        println!("{}", d);
    }
}