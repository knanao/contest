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

use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    let (n, q): (usize, usize) = (sc.next(), sc.next());
    let mut ab: Vec<Vec<usize>> = vec![vec![]; 200005];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = (sc.next(), sc.next());
        ab[a - 1].push(b - 1);
        ab[b - 1].push(a - 1);
    }

    let mut ans = vec![0; n];
    for _ in 0..q {
        let (p, x): (usize, usize) = (sc.next(), sc.next());
        ans[p - 1] += x;
    }
    dfs(&mut ans, &ab, 0, -1);

    for i in 0..n {
        print!("{}", ans[i]);
        if i != n - 1 {
            print!(" ");
        }
    }
    println!("");
}

fn dfs(ans: &mut Vec<usize>, ab: &Vec<Vec<usize>>, x: usize, p: i32) {
    for (_, &v) in ab[x].iter().enumerate() {
        if v as i32 == p {
            continue;
        }
        ans[v] += ans[x];
        dfs(ans, ab, v, x as i32);
    }
}
