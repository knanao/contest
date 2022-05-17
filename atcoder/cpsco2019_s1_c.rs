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

    let (n, k) = (sc.next(), sc.next());
    let a: Vec<u64> = sc.vec(n);

    let ret = dfs(&a, k, 0, 0);
    println!("{}", ret);
}

fn dfs(a: &[u64], k: usize, sum: u64, v: usize) -> u64 {
    if k == 0 {
        let mut c = 0;
        let mut s = sum;
        while s > 0 {
            let r = s % 10;
            c += r / 5 + r % 5;
            s /= 10;
        }
        return c;
    }

    let mut ret = std::u64::MAX;
    for i in v..a.len() {
        ret = std::cmp::min(ret, dfs(a, k - 1, sum + a[i], i + 1));
    }
    ret
}
