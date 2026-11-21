use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

const MOD: u32 = 666_013;

/// Fast reader: reads the file in chunks and parses non-negative integers.
/// Does not load the whole file into memory (it can be tens of MB).
struct Scanner<R: Read> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
}

impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Self {
        Scanner { reader, buf: vec![0; 1 << 16], pos: 0, len: 0 }
    }

    fn byte(&mut self) -> Option<u8> {
        if self.pos == self.len {
            self.len = self.reader.read(&mut self.buf).ok()?;
            self.pos = 0;
            if self.len == 0 {
                return None;
            }
        }
        let b = self.buf[self.pos];
        self.pos += 1;
        Some(b)
    }

    fn next(&mut self) -> u64 {
        let mut b = self.byte().expect("unexpected end of input");
        while !b.is_ascii_digit() {
            b = self.byte().expect("unexpected end of input");
        }
        let mut n = 0u64;
        while b.is_ascii_digit() {
            n = n * 10 + (b - b'0') as u64;
            match self.byte() {
                Some(c) => b = c,
                None => break,
            }
        }
        n
    }
}

/// Binary indexed tree (Fenwick), 1-indexed, with sums modulo MOD.
struct Fenwick {
    t: Vec<u32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { t: vec![0; n + 1] }
    }

    /// Adds `val` at position `i` (1..=n).
    fn add(&mut self, mut i: usize, val: u32) {
        while i < self.t.len() {
            self.t[i] = (self.t[i] + val) % MOD;
            i += i & i.wrapping_neg();
        }
    }

    /// Sum of positions 1..=i.
    fn prefix(&self, mut i: usize) -> u32 {
        let mut s = 0u32;
        while i > 0 {
            s = (s + self.t[i]) % MOD;
            i -= i & i.wrapping_neg();
        }
        s
    }
}

fn main() {
    let input = File::open("subsiruri3.in").expect("cannot open subsiruri3.in");
    let output = File::create("subsiruri3.out").expect("cannot create subsiruri3.out");
    let mut sc = Scanner::new(BufReader::new(input));
    let mut out = BufWriter::new(output);

    let tests = sc.next();
    for _ in 0..tests {
        let n = sc.next() as usize;
        let k = sc.next() as i64;
        let x: Vec<u32> = (0..n).map(|_| sc.next() as u32).collect();

        // Coordinate compression: distinct values, sorted.
        let mut v = x.clone();
        v.sort_unstable();
        v.dedup();

        let mut fw = Fenwick::new(v.len());
        let mut total = 0u32;

        for &val in &x {
            // p = index of val in v (0-based); in the Fenwick tree it is p + 1.
            let p = v.binary_search(&val).unwrap();
            // s = first index with v[s] >= val - K (0-based).
            let left = val as i64 - k;
            let s = v.partition_point(|&a| (a as i64) < left);

            // Sum of dp over values in [val - K, val] = prefix(p+1) - prefix(s).
            let sum = (fw.prefix(p + 1) + MOD - fw.prefix(s)) % MOD;
            let dp = (1 + sum) % MOD;

            fw.add(p + 1, dp);
            total = (total + dp) % MOD;
        }

        writeln!(out, "{}", total).unwrap();
    }
}
