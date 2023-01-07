use proconio::input;
use proconio::marker::Chars;
use proconio::fastout;

struct Hash {
    modulo: i64,
    powers: Vec<i64>,
    hashes: Vec<i64>
}

impl Hash {
    fn new(s: &Vec<char>, base: i64, modulo: i64) -> Self {
        let n = s.len();
        let mut powers = vec![1i64; n + 1];
        for i in 1..=n {
            powers[i] = (powers[i - 1] * base) % modulo;
        }
        // charcodesもhashesも1-originとする（sは0-origin）
        let mut charcodes = vec![0i64; n + 1];
        for i in 1..=n {
            charcodes[i] = s[i - 1] as i64 + 1 - 'a' as i64;
        }
        let mut hashes = vec![0i64; n + 1];
        for i in 1..=n {
            hashes[i] = (hashes[i - 1] * base + charcodes[i]) % modulo;
        }
        Self { modulo, powers, hashes }
    }
    fn get(&self, l: usize, r: usize) -> i64 {
        let mut hash = self.hashes[r] - ((self.hashes[l - 1] * self.powers[r - (l - 1)]) % self.modulo);
        if hash < 0 {
            hash += self.modulo;
        }
        hash
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _: usize,
        Q: usize,
        S: Chars
    }
    let B = 100i64;
    let M = (1i64 << 31) - 1; // 素数（2147483647）
    let hash = Hash::new(&S, B, M);
    for _ in 0..Q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize
        }
        if hash.get(a, b) == hash.get(c, d) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
