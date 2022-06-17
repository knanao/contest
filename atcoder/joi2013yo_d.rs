use std::cmp::Ordering;
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

#[allow(dead_code)]
const INF: usize = 1 << 60;

fn main() {
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    let (d, n): (usize, usize) = (sc.next(), sc.next());
    let mut t = vec![0; d + 1];
    for i in 1..=d {
        t[i] = sc.next();
    }
    let mut abc: Vec<(usize, usize, usize)> = vec![(0, 0, 0); n];
    for i in 0..n {
        abc[i] = (sc.next(), sc.next(), sc.next());
    }

    let mut dp = vec![vec![0; n]; d + 1];
    for i in 1..d {
        for j in 0..n {
            let (last_a, last_b, last_c) = abc[j];
            if last_a > t[i] || last_b < t[i] {
                continue;
            }
            for k in 0..n {
                let (a, b, c) = abc[k];
                if a <= t[i + 1] && b >= t[i + 1] {
                    let cost = (c as i32 - last_c as i32).abs() as usize;
                    dp[i + 1][k] = std::cmp::max(dp[i + 1][k], cost + dp[i][j]);
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = std::cmp::max(ans, dp[d][i]);
    }
    println!("{}", ans);
}
