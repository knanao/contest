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

    let mut s: String = sc.next();
    let n: i32 = sc.next();

    for _ in 0..n {
        let cmd: String = sc.next();
        let a: usize = sc.next();
        let b: usize = sc.next::<usize>() + 1;

        let t = s.clone();

        match cmd.as_ref() {
            "print" => {
                println!("{}", &s[a..b]);
            }
            "reverse" => {
                let x = &t[..a];
                let y = &t[a..b].chars().rev().collect::<String>();
                let z = &t[b..];
                s = format!("{}{}{}", x, y, z);
            }
            "replace" => {
                let x = &t[..a];
                let y = sc.next::<String>();
                let z = &t[b..];
                s = format!("{}{}{}", x, y, z);
            }
            _ => {
                return;
            }
        }
    }
}
