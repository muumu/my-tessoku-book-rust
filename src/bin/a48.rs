use itertools::Itertools;
use rand::Rng;
use proconio::{input, fastout};

const LOOP_NUM: usize = 200000;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(isize, isize); N]
    }
    let (X, Y): (Vec<_>, Vec<_>) = XY.into_iter().unzip();
    let mut route = vec![0; N + 1];
    for i in 1..N {
        route[i] = i;
    }
    let mut min_dist = 0.0;
    for i in 0..N {
        min_dist += distance(X[route[i]], Y[route[i]], X[route[i + 1]], Y[route[i + 1]]);
    }
    let mut rng = rand::thread_rng();
    for t in 0..LOOP_NUM {
        let mut left = rng.gen_range(1, N + 1);
        let mut right = rng.gen_range(1, N + 1);
        if left == right {
            continue;
        } else if left > right {
            std::mem::swap(&mut left, &mut right);
        }
        let reversed = route[left..right].iter().rev().map(|&x| x).collect_vec();
        let r = [&route[0..left], &reversed[..], &route[right..]].concat();
        let mut dist = 0.0;
        for i in 0..N {
            dist += distance(X[r[i]], Y[r[i]], X[r[i + 1]], Y[r[i + 1]]);
        }
        // 焼きなまし
        let T = 30.0 - 28.0 * t as f64 / LOOP_NUM as f64;
        let probability = ((min_dist - dist).min(0.0) / T).exp();
        let random = rng.gen_range(0.0, 1.0);
        if random < probability {
            min_dist = dist;
            route = r;
        }
    }
    for ans in route {
        println!("{}", ans + 1); // 0-origin -> 1-origin
    }
}

fn distance(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt()
}