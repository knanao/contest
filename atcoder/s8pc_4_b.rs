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

    let (n, k): (usize, usize) = (sc.next(), sc.next());
    let a: Vec<u64> = sc.vec(n);

    let mut ret = std::u64::MAX;
    for i in 0..(1 << n) {
        if (i as u64).count_ones() < k as u32 {
            continue;
        }

        let mut s = 0;
        let mut m = 0;
        let mut c = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                if m >= a[j] {
                    s += m - a[j] + 1;
                    m += 1;
                } else {
                    m = a[j];
                }
                c += 1;
            } else {
                if m < a[j] {
                    m = a[j];
                    c += 1;
                }
            }
        }
        if c >= k {
            ret = std::cmp::min(ret, s);
        }
    }
    println!("{}", ret);
}
