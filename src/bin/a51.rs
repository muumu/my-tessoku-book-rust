use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize
    }
    let mut stack: Vec<String> = vec![];
    for _ in 0..Q {
        input! {
            query_id: usize,
        }
        if query_id == 1 {
            input! {
                title: String
            }
            stack.push(title);
        } else if query_id == 2 {
            println!("{}", stack[stack.len() - 1]);
        } else {
            stack.pop();
        }
    }
}
