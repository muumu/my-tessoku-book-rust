use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        N: usize,
        LRH: [(usize, usize, usize); N]
    }
    let mut max = vec![24; D];
    for i in 0..N {
        for j in LRH[i].0..=LRH[i].1 {
            max[j - 1] = std::cmp::min(max[j - 1], LRH[i].2);
        }
    }
    println!("{}", max.iter().sum::<usize>());
}
