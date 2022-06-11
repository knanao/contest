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
    let mut v: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n {
        let u: usize = sc.next();
        let k: usize = sc.next();
        for _ in 0..k {
            let vi: usize = sc.next();
            v[u - 1].push(vi - 1);
        }
    }

    let mut seen = vec![(0, 0); n];
    dfs(&mut seen, &v, 0, &mut 0);
    for i in 0..n {
        println!("{} {} {}", i + 1, seen[i].0, seen[i].1);
    }
}

fn dfs(seen: &mut Vec<(usize, usize)>, v: &Vec<Vec<usize>>, pos: usize, time: &mut usize) {
    *time += 1;
    seen[pos].0 = *time;

    for (_, vi) in v[pos].iter().enumerate() {
        if seen[*vi].1 != 0 {
            continue;
        }
        dfs(seen, v, *vi, time);
    }
    *time += 1;
    seen[pos].1 = *time;
}
