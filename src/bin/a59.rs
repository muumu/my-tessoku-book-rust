use proconio::{input, fastout};

struct SegmentTree {
    arr: Vec<i64>, // // 1-originとする
    size: usize // リーフ（最末端）の要素数 == リーフ以外の要素数 - 1
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {arr: vec![0; 2 * size], size}
    }
    fn size(&self) -> usize {
        self.size
    }
    // posは1-originとする
    fn update(&mut self, pos: usize, x: i64) {
        let mut p = self.size - 1 + pos;
        self.arr[p] = x;
        while p >= 2 {
            p /= 2;
            self.arr[p] = self.arr[2 * p] + self.arr[2 * p + 1];
        }
    }
    // [l, r)が最終的に求める区間で、[a, b)が今回処理対象とする区間、uは[a, b)を区間として持つarrの添字
    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> i64 {
        if a >= r || b <= l {
            return 0;
        }
        if a >= l && b <= r {
            return self.arr[u];
        }
        let left_child = self.query(l, r, a, (a + b) / 2, 2 * u);
        let right_child = self.query(l, r, (a + b) / 2, b, 2 * u + 1);
        return left_child + right_child;
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize
    }
    let mut tree = SegmentTree::new(N);
    for _ in 0..Q {
        input! {
            query_id: usize,
        }
        if query_id == 1 {
            input! {
                pos: usize,
                x: i64
            }
            tree.update(pos, x);
        } else {
            input! {
                l: usize,
                r: usize
            }
            let ans = tree.query(l, r, 1, tree.size() + 1, 1);
            println!("{}", ans);
        }
    }
}
