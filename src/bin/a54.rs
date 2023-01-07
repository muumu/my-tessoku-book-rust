use proconio::input;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize
    }
    let mut mp: HashMap<String, usize> = HashMap::new();
    for _ in 0..Q {
        input! {
            query_id: usize,
        }
        if query_id == 1 {
            input! {
                name: String,
                score: usize
            }
            mp.insert(name, score);
        } else {
            input! {
                name: String
            }
            println!("{}", mp.get(&name).unwrap());
        }
    }
}
