use proconio::input;
use proconio::marker::Usize1;

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
    let mut visited = vec![false; N];
    let mut stack = vec![];
    stack.push(0);
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        visited[v] = true;
        for &next_v in &graph[v] {
            if !visited[next_v] {
                stack.push(next_v);
            }
        }
    }
    if visited.iter().all(|&x| x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}