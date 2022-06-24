use std::cmp::*;
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

    let (n, m): (usize, usize) = (sc.next(), sc.next());
    let mut dst = vec![vec![(0, 0); n]; n];
    let mut time = vec![0; m];
    for i in 0..m {
        let (s, t, d): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
        dst[s - 1][t - 1] = (i, d);
        dst[t - 1][s - 1] = (i, d);
        time[i] = sc.next();
    }

    let mut dp = vec![vec![INF; n]; 1 << n];
    dp[0][0] = 0;
    let mut num = vec![vec![0; n]; 1 << n];
    num[0][0] = 1;
    for i in 0..1 << n {
        for j in 0..n {
            if i >> j & 1 == 0 {
                continue;
            }
            for k in 0..n {
                if k != 0 && i >> k & 1 == 0 {
                    continue;
                }
                let (ii, d) = dst[k][j];
                if d == 0 {
                    continue;
                }
                if dp[i & !(1 << j)][k] + d > time[ii] {
                    continue;
                }
                if dp[i][j] > dp[i & !(1 << j)][k] + d {
                    dp[i][j] = dp[i & !(1 << j)][k] + d;
                    num[i][j] = num[i & !(1 << j)][k];
                } else if dp[i][j] == dp[i & !(1 << j)][k] + d {
                    num[i][j] += num[i & !(1 << j)][k];
                }
            }
        }
    }
    if dp[(1 << n) - 1][0] != INF {
        println!("{} {}", dp[(1 << n) - 1][0], num[(1 << n) - 1][0]);
    } else {
        println!("IMPOSSIBLE");
    }
}
