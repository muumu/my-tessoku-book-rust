use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }
    for i in (0..10).rev() {
        print!("{}", (N >> i) % 2);
    }
    println!();
}

// 別解
// fn main() {
//     input! {
//         N: usize
//     }
//     println!("{:0>1$b}", N, 10);
// }