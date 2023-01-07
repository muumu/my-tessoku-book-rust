use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize
    }
    let mut G = vec![vec![]; N];
    let scale: usize = 10000;
    for _ in 0..M {
        input! {
            A: Usize1, // 頂点番号は0-originとする
            B: Usize1,
            C: usize,
            D: usize
        }
        G[A].push(Edge {to: B, weight: C * scale - D});
        G[B].push(Edge {to: A, weight: C * scale - D});
    }
    let mut queue = BinaryHeap::new();
    let mut dist = vec![std::usize::MAX; N];
    let mut settled = vec![false; N];
    dist[0] = 0;
    queue.push(Reverse((dist[0], 0)));
    while !queue.is_empty() {
        let Reverse((_, v)) = queue.pop().unwrap();
        if settled[v] {
            continue;
        }
        settled[v] = true;
        for &edge in &G[v] {
            if dist[v] + edge.weight < dist[edge.to] {
                dist[edge.to] = dist[v] + edge.weight;
                queue.push(Reverse((dist[edge.to], edge.to)));
            }
        }
    }
    let distance = (dist[N - 1] + scale - 1) / scale;
    let tree_num = distance * scale - dist[N - 1];
    println!("{} {}", distance, tree_num);
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
    weight: usize
}