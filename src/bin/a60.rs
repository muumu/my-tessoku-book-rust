use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [isize; N]
    }
    let mut ans: Vec<isize> = vec![-1; N];
    let mut tops: Vec<(usize, isize)> = vec![(0, A[0])];
    for i in 1..N {
        if A[i] < A[i - 1] {
            ans[i] = i as isize; // 1-originに変換(i - 1 -> i)
        }
        else {
            while !tops.is_empty() {
                if A[i] < tops[tops.len() - 1].1 {
                    ans[i] = tops[tops.len() - 1].0 as isize + 1; // 1-originに変換
                    break;
                } else {
                    tops.pop();
                }
            }
            if tops.is_empty() {
                ans[i] = -1;
            }
        }
        tops.push((i, A[i]));
    }
    let ans = ans.iter().join(" ");
    println!("{}", ans);
}
