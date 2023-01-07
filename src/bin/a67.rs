use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut ABC: [(Usize1, Usize1, usize); M]
    }
    let mut uf = UnionFind::new(N);
    ABC.sort_by(|a, b| a.2.cmp(&b.2));
    let mut ans = 0;
    for &abc in &ABC {
        let (a, b, c) = abc;
        if !uf.same(a, b) {
            uf.unite(a, b);
            ans += c;
        }
    }
    println!("{}", ans);
}

struct UnionFind {
    par: Vec<Option<usize>>,
    siz: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { par: vec![None; n], siz: vec![1; n]}
    }
    fn root(&self, x: usize) -> usize {
        let mut some = Some(x);
        loop {
            let i = some.unwrap();
            if self.par[i] == None {
                return i;
            }
            some = self.par[i];
        }
    }
    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            return;
        }
        if self.siz[root_u] < self.siz[root_v] {
            self.par[root_u] = Some(root_v);
            self.siz[root_v] = self.siz[root_u] + self.siz[root_v];
        } else {
            self.par[root_v] = Some(root_u);
            self.siz[root_u] = self.siz[root_u] + self.siz[root_v];
        }
    }
    fn same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}