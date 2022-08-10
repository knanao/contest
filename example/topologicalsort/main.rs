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

    let (n, m): (usize, usize) = (sc.next(), sc.next());
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let (a, b): (usize, usize) = (sc.next(), sc.next());
        g[a].push(b);
    }

    let mut seen: Vec<bool> = vec![false; n];
    let mut order: Vec<usize> = Vec::new();

    for i in 0..n {
        if seen[i] {
            continue;
        }
        dfs(i, &g, &mut seen, &mut order);
    }

    order.reverse();
    println!("{:?}", order);
}

fn dfs(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
    seen[v] = true;

    for i in &g[v] {
        if seen[*i] {
            continue;
        }
        dfs(*i, g, seen, order);
    }

    order.push(v);
}
