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

    let (n, m, q): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());

    let mut v: Vec<(usize, usize, u32, u32)> = vec![(0, 0, 0, 0); q];
    for i in 0..q {
        v[i] = (sc.next(), sc.next(), sc.next(), sc.next());
    }

    let mut ai = vec![0; n];
    let ret = dfs(&mut ai, &v, 0, m);
    println!("{}", ret);
}

fn dfs(ai: &mut Vec<u32>, v: &Vec<(usize, usize, u32, u32)>, i: usize, m: usize) -> u32 {
    if ai.len() == i {
        let mut ret = 0;
        for &(a, b, c, d) in v {
            if ai[b - 1] - ai[a - 1] == c {
                ret += d;
            }
        }
        return ret;
    }

    let mut ret = 0;
    let t = if i == 0 { 1 } else { ai[i - 1] };
    for j in t..=m as u32 {
        ai[i] = j;
        ret = std::cmp::max(ret, dfs(ai, v, i + 1, m));
    }
    ret
}
