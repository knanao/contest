use std::io::*;
use std::str::FromStr;

struct Scanner<R: Read> {
    reader: R,
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }

    fn read<T: FromStr>(&mut self) -> Option<T> {
        let token = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            token.parse::<T>().ok()
        }
    }

    fn next<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.read() {
            s
        } else {
            writeln!(stderr(), "Terminated with EOF").unwrap();
            std::process::exit(0);
        }
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }

    fn chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }

    fn char(&mut self) -> char {
        self.chars()[0]
    }
}

fn main() {
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    let n: usize = sc.next();
    let m: usize = sc.next();
    let x: u64 = sc.next();

    let mut c: Vec<u64> = vec![0; n];
    let mut a: Vec<Vec<u64>> = vec![vec![0; m]; n];
    for i in 0..n {
        c[i] = sc.next();
        for j in 0..m {
            a[i][j] = sc.next();
        }
    }

    let mut ret = std::u64::MAX;
    'outer: for i in 0..(1 << n) {
        let mut t = vec![0; m];
        let mut ct = 0;
        for j in 0..n {
            if (i >> j & 1) == 1 {
                for k in 0..m {
                    t[k] += a[j][k];
                }
                ct += c[j];
            }
        }
        for j in t {
            if j < x {
                continue 'outer;
            }
        }
        ret = std::cmp::min(ret, ct);
    }
    if ret == std::u64::MAX {
        println!("-1")
    } else {
        println!("{}", ret)
    };
}
