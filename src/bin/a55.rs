use proconio::input;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize
    }
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for _ in 0..Q {
        input! {
            query_id: usize,
        }
        if query_id == 1 {
            input! {
                x: usize
            }
            set.insert(x);
        } else if query_id == 2 {
            input! {
                x: usize
            }
            set.remove(&x);
        } else {
            input! {
                x: usize
            }
            if let Some(&value) = set.range(x..).next() {
                println!("{}", value);
            } else {
                println!("-1");
            }
        }
    }
}
