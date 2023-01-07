use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize
    }
    let mut mf = MaximumFlow::new(N);
    for _ in 0..M {
        input! {
            A: Usize1,
            B: Usize1,
            C: usize
        }
        mf.add_edge(A, B, C);
    }
    println!("{}", mf.max_flow(0, N - 1))
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize
}

struct MaximumFlow {
    graph: Vec<Vec<Edge>>
}

impl MaximumFlow {
    fn new(n: usize) -> Self {
        Self { graph: vec![vec![]; n]}
    }
    // 頂点a -> b (上限cリットル/秒)
    fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let pos_a = self.graph[a].len();
        let pos_b = self.graph[b].len();
        self.graph[a].push(Edge { to: b, cap: c, rev: pos_b});
        self.graph[b].push(Edge { to: a, cap: 0, rev: pos_a});
    }
    fn dfs(&mut self, pos: usize, goal: usize, f: usize, used: &mut Vec<bool>) -> usize {
        if pos == goal {
            return f;
        }
        used[pos] = true;
        for i in 0..self.graph[pos].len() {
            let to = self.graph[pos][i].to;
            if self.graph[pos][i].cap == 0 || used[to] {
                continue;
            }
            let next_f = std::cmp::min(f, self.graph[pos][i].cap);
            let flow = self.dfs(to, goal, next_f, used);
            if flow >= 1 {
                self.graph[pos][i].cap -= flow;
                let rev = self.graph[pos][i].rev;
                self.graph[to][rev].cap += flow;
                return flow;
            }
        }
        return 0;
    }
    fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0;
        loop {
            let mut used = vec![false; self.graph.len()];
            let f = self.dfs(s, t, std::usize::MAX, &mut used);
            if f == 0 {
                break;
            }
            total_flow += f;
        }
        return total_flow;
    }
}