use proconio::{input, fastout};
use proconio::marker::Usize1;

const N: usize = 20;
const WIDTH: usize = 10000;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        T: usize,
        PQR: [(Usize1, Usize1, Usize1); T]
    }
    let mut beam: Vec<Vec<State>> = vec![vec![]; T + 1];
    let mut answer = vec!['A'; T];
    beam[0] = vec![State::new()];
    for i in 1..=T {
        for j in 0..beam[i - 1].len() {
            let (P, Q, R) = PQR[i - 1];
            for &op in ['A', 'B'].iter() {
                let mut state = beam[i - 1][j].clone();
                state.update(op, j, P, Q, R);
                beam[i].push(state);
            }
        }
        beam[i].sort_by(|a, b| b.score.cmp(&a.score));
        if beam[i].len() > WIDTH {
            beam[i].drain(WIDTH..);
        }
    }
    let mut pos = 0;
    for i in (1..=T).rev() {
        answer[i - 1] = beam[i][pos].last_op; // answerは0-origin
        pos = beam[i][pos].last_pos;
    }
    for op in answer {
        println!("{}", op);
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    score: isize,
    x: [isize; N],
    last_op: char,
    last_pos: usize
}

impl State {
    fn new() -> Self {
        Self { score: 0, x: [0; N], last_op: 'A', last_pos: 0 }
    }
    fn update(&mut self, op: char, pos: usize, p: usize, q: usize, r: usize) {
        self.last_op = op;
        self.last_pos = pos;
        let diff = if op == 'A' { 1 } else { -1 };
        for &i in [p, q, r].iter() {
            self.x[i] += diff;
        }
        self.score += self.x.iter().map(|&x| if x == 0 { 1 } else { 0 }).sum::<isize>();
    }
}