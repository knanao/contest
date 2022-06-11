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

    let (d, n, m): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let dn: Vec<usize> = sc.vec(n - 1);
    let k: Vec<usize> = sc.vec(m);

    let mut s: Vec<usize> = vec![];
    s.push(0);
    s.push(d);
    dn.iter().for_each(|x| s.push(*x));
    s.sort();

    let mut ans = 0;
    for i in 0..m {
        let p = s.binary_search(&k[i]).unwrap_or_else(|x| x);
        ans += std::cmp::min(s[p] - k[i], k[i] - s[p - 1]);
    }
    println!("{}", ans);
}
