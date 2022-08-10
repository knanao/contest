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

    let m: usize = sc.next();
    let mut w: Vec<(i32, i32)> = vec![(0, 0); m];
    for i in 0..m {
        w[i] = (sc.next(), sc.next());
    }
    let n: usize = sc.next();
    let mut p: Vec<(i32, i32)> = vec![(0, 0); n];
    let mut v: Vec<(i32, i32)> = vec![(0, 0); n];
    for i in 0..n {
        p[i] = (sc.next(), sc.next());
        v[i] = (p[i].0 - w[0].0, p[i].1 - w[0].1);
    }

    for j in 0..n {
        let mut ok = true;
        for i in 1..m {
            let (x, y) = (w[i].0 + v[j].0, w[i].1 + v[j].1);
            if p.contains(&(x, y)) {
                continue;
            }
            ok = false;
        }
        if ok {
            println!("{} {}", v[j].0, v[j].1);
            return;
        }
    }
}
