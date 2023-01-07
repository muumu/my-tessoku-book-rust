use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut nums: Vec<i64> = vec![0; 101];
    for i in 0..N {
        nums[A[i]] += 1;
    }
    let mut ans = 0;
    for i in 0..=100 {
        ans += nums[i] * (nums[i] - 1) * (nums[i] - 2) / 6;
    }
    println!("{}", ans);
}
