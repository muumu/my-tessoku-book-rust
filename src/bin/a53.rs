use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize
    }
    let mut heap = BinaryHeap::new();
    for _ in 0..Q {
        input! {
            query_id: usize
        }
        if query_id == 1 {
            input! {
                price: usize
            }
            heap.push(Reverse(price));
        } else if query_id == 2 {
            if let Some(Reverse(price)) = heap.peek() {
                println!("{}", price);
            }
        } else {
            heap.pop();
        }
    }
}
