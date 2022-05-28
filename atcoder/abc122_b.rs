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
    let s: String = sc.next();

    let cs: Vec<char> = s.chars().collect();
    let mut ans: usize = 0;
    for (i, c) in cs.iter().enumerate() {
        if *c != 'A' && *c != 'C' && *c != 'G' && *c != 'T' {
            continue;
        }
        let mut count: usize = 1;
        for &j in cs[i + 1..].iter() {
            if j != 'A' && j != 'C' && j != 'G' && j != 'T' {
                break;
            }
            count += 1;
        }
        ans = std::cmp::max(ans, count);
    }
    println!("{}", ans);
}
