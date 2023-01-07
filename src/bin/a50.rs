extern crate rand;
use rand::Rng;
use std::time::Instant;
use proconio::{input, fastout};
use std::cmp::{max, min};

const N: usize = 100;
const Q: usize = 1000;

const TIME_LIMIT: u128 = 5950;
const DELTA_X: isize = 9;
const DELTA_Y: isize = 9;
const DELTA_H: isize = 19;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: [[isize; N]; N]
    }
    let mut hill_climbing = HillClimbing::new(A);
    hill_climbing.init();
    hill_climbing.execute();
    println!("{}", Q);
    for op in &hill_climbing.ops {
        println!("{} {} {}", op.x, op.y, op.h);
    }
}

#[derive(Clone, Copy, Debug)]
struct Operation {
    x: isize,
    y: isize,
    h: isize
}

impl Operation {
    fn new() -> Self {
        Self { x: 0, y: 0, h: 0 }
    }
}

struct HillClimbing {
    goal: Vec<Vec<isize>>,
    board: Vec<Vec<isize>>,
    ops: Vec<Operation>
}

impl HillClimbing {
    fn new(goal: Vec<Vec<isize>>) -> Self {
        Self { goal, board: vec![vec![0; N]; N], ops: vec![Operation::new(); Q]}
    }
    fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for t in 0..Q {
            self.ops[t].x = rng.gen_range(0, N as isize);
            self.ops[t].y = rng.gen_range(0, N as isize);
            self.ops[t].h = 1;
            // Hが1のときは(X, Y)のマス目に1を足すだけで良い
            self.board[self.ops[t].y as usize][self.ops[t].x as usize] += 1;
        }
    }
    fn change(&mut self, t: usize, new_op: &Operation) {
        for i in 0..N {
            for j in 0..N {
                let diff_x = (self.ops[t].x - i as isize).abs();
                let diff_y = (self.ops[t].y - j as isize).abs();
                self.board[j][i] -= max(0, self.ops[t].h - diff_x - diff_y);
            }
        }
        self.ops[t] = *new_op;
        for i in 0..N {
            for j in 0..N {
                let diff_x = (self.ops[t].x - i as isize).abs();
                let diff_y = (self.ops[t].y - j as isize).abs();
                self.board[j][i] += max(0, self.ops[t].h - diff_x - diff_y);
            }
        }
    }
    fn score(&self) -> isize {
        let mut sum = 0;
        for i in 0..N {
            for j in 0..N {
                sum += (self.goal[i][j] - self.board[i][j]).abs();
            }
        }
        200000000 - sum
    }
    fn execute(&mut self) {
        let mut current_score = self.score();
        let mut rng = rand::thread_rng();
        let now = Instant::now();
        while now.elapsed().as_millis() < TIME_LIMIT {
            let mut new_op = Operation::new();
            let t = rng.gen_range(0, Q);
            let old_op = self.ops[t];
            let low = max(0, self.ops[t].x - DELTA_X);
            let high = min(N as isize - 1, self.ops[t].x + DELTA_X);
            new_op.x = rng.gen_range(low, high + 1);
            let low = max(0, self.ops[t].y - DELTA_Y);
            let high = min(N as isize - 1, self.ops[t].y + DELTA_Y);
            new_op.y = rng.gen_range(low, high + 1);
            let low = max(1, self.ops[t].h - DELTA_H);
            let high = min(N as isize, self.ops[t].h + DELTA_H);
            new_op.h = rng.gen_range(low, high + 1);
            self.change(t, &new_op);
            let score = self.score();
            if score >= current_score {
                current_score = score;
            } else {
                self.change(t, &old_op);
            }
        }
    }
}