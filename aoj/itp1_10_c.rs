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

    fn char(&mut self) -> char {
        self.next::<String>().chars().next().unwrap()
    }
}

fn main() {
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    loop {
        let n = sc.next::<u32>();
        if n == 0 {
            break;
        }

        let mut s1: f64 = 0.;
        let mut s2: f64 = 0.;

        for _ in 0..n {
            let a = sc.next::<f64>();
            s1 += a;
            s2 += a.powi(2);
        }

        let n = n as f64;
        let v = (s2 / n) - (s1 / n).powi(2);
        println!("{:.8}", v.sqrt());
    }
}
