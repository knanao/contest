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

    let n = sc.next::<usize>();
    let x = sc.vec::<f64>(n);
    let y = sc.vec::<f64>(n);

    let abs: Vec<f64> = x.iter().zip(y).map(|(x, y)| (x - y).abs()).collect();

    for i in 1..4 {
        let p = i as f64;
        let s: f64 = abs.iter().map(|x| x.powf(p)).sum();
        println!("{:.6}", s.powf(1. / p));
    }

    let s: f64 = abs.iter().cloned().fold(0., f64::max);
    println!("{:.6}", s);
}
