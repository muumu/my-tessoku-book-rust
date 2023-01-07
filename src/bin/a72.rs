use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        c: [Chars; H]
    }
    let powH = 2usize.pow(H as u32);
    let mut initial_cnt = 0; // 初期状態での黒マス数
    for i in 0..H {
        for j in 0..W {
            if c[i][j] == '#' {
                initial_cnt += 1;
            }
        }
    }
    let mut max = 0;
    // state: 黒く塗り替える行を1、塗り替えない行を0としたビット表現
    for state in 0..powH {
        if state.count_ones() as usize > K {
            continue;
        }
        let remainingK = K - state.count_ones() as usize;
        let mut sum = 0;
        let mut cnt = vec![0; W];
        for i in 0..H {
            if (state & (1 << i)) > 0 {
                for j in 0..W {
                    if c[i][j] == '.' {
                        sum += 1;
                    }
                }
            } else if remainingK > 0 {
                for j in 0..W {
                    if c[i][j] == '.' {
                        cnt[j] += 1;
                    }
                }
            }
        }
        if remainingK > 0 {
            cnt.sort();
            cnt.reverse();
            sum += cnt[..remainingK].iter().sum::<usize>();
        }
        max = std::cmp::max(max, sum);
    }
    println!("{}", max + initial_cnt);
}
