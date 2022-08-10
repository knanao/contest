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

    let r: usize = sc.next();
    let c: usize = sc.next();
    let mut a = vec![0; c];

    for i in 0..r {
        for j in 0..c {
            let v: usize = sc.next();
            a[j] += (1 << i) * v;
        }
    }

    let mut ans = 0;
    for i in 0..(1 << r) {
        ans = std::cmp::max(
            ans,
            a.iter()
                .map(|&x| {
                    let y: usize = (x ^ i).count_ones() as usize;
                    std::cmp::max(r - y, y)
                })
                .sum(),
        )
    }

    println!("{}", ans);
}
