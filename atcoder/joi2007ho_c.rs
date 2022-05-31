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
    let mut xy: Vec<(i32, i32)> = vec![];
    let mut hs: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    for i in 0..n {
        xy.push((sc.next(), sc.next()));
        hs.insert(xy[i]);
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let v = (xy[j].0 - xy[i].0, xy[j].1 - xy[i].1);
            if hs.contains(&(xy[i].0 + v.1, xy[i].1 - v.0))
                && hs.contains(&(xy[j].0 + v.1, xy[j].1 - v.0))
            {
                ans = std::cmp::max(ans, v.0 * v.0 + v.1 * v.1);
            }
        }
    }
    println!("{}", ans);
}
