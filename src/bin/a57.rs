use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [usize; N]
    }
    A.insert(0, 0); // Aの先頭に0を挿入して1-originに変換
    // dp[i][j]: 穴jにいたときから2^i日後の穴を格納する配列
    // Y < 10^9 < 2^30のためiの最大値は29で十分
    let mut dp = vec![vec![0; N + 1]; 30];
    for j in 1..=N {
        dp[0][j] = A[j]; // 2^0日後は1日後なのでA[j]が直接入る
    }
    for i in 1..=29 {
        for j in 1..=N {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }
    for _ in 0..Q {
        input! {
            X: usize,
            Y: usize
        }
        let mut digit = 0;
        let mut ans = X;
        while (Y >> digit) > 0 {
            if (Y >> digit) & 1 == 1 {
                ans = dp[digit][ans];
            }
            digit += 1;
        }
        println!("{}", ans);
    }
}
