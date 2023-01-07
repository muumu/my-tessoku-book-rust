use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }
    let mut low = 0;
    let mut high = 1_000_000_000;
    // lowはK枚未満しか印刷されない秒数
    // highはK枚以上印刷される秒数
    // lowとhighが1秒差になるまで範囲を狭めていくと、
    // highが最初にK以上の枚数が印刷される秒数となる
    while low < high {
        let mid = (low + high) / 2;
        let mut printed_num = 0;
        for a in &A {
            printed_num += mid / a;
        }
        if printed_num >= K {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    println!("{}", low);
}
