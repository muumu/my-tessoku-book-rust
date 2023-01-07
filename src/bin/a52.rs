use proconio::input;
use std::collections::VecDeque;

#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize
    }
    let mut queue: VecDeque<String> = VecDeque::new();
    for _ in 0..Q {
        input! {
            query_id: usize,
        }
        if query_id == 1 {
            input! {
                title: String
            }
            queue.push_back(title);
        } else if query_id == 2 {
            println!("{}", queue.front().unwrap());
        } else {
            queue.pop_front();
        }
    }
}
