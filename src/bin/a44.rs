use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut arr = ReversibleArray::new(N);
    for _ in 0..Q {
        input! {
            q: usize
        }
        if q == 1 {
            input! {
                x: usize,
                y: usize
            }
            arr.set(x, y);
        } else if q == 2 {
            arr.reverse();
        } else {
            input! {
                x: usize
            }
            println!("{}", arr.get(x));
        }
    }
}

struct ReversibleArray {
    a: Vec<usize>,
    order: bool
}

impl ReversibleArray {
    fn new(n: usize) -> ReversibleArray {
        Self { a: (1..=n).collect_vec(), order: true }
    }
    fn get(&self, i: usize) -> usize {
        if self.order { self.a[i - 1] } else { self.a[self.a.len() - i] }
    }
    fn set(&mut self, i: usize, value: usize) {
        if self.order {
            self.a[i - 1] = value;
        } else {
            let len = self.a.len();
            self.a[len - i] = value;
        }
        
    }
    fn reverse(&mut self) {
        self.order = if self.order { false } else { true };
    }
}
