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

    let (n, k): (usize, usize) = (sc.next(), sc.next());
    let mut d: Vec<usize> = vec![0; n];
    for _ in 0..k {
        let (a, b): (usize, usize) = (sc.next(), sc.next());
        d[a - 1] = b;
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 4]; 4]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..=3 {
            for k in 0..=3 {
                for l in 1..=3 {
                    if d[i] != 0 && d[i] != l {
                        continue;
                    }
                    if j != l || j != k {
                        dp[i + 1][l][j] += dp[i][j][k];
                        dp[i + 1][l][j] %= 10000;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..4 {
        for j in 0..4 {
            ans += dp[n][i][j];
            ans %= 10000;
        }
    }
    println!("{}", ans);
}
