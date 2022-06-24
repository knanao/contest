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
    let mut a = vec![0; n];
    let mut c = vec![0; 20];
    let mut l = vec![vec![0; 101010]; 20];
    for i in 0..n {
        let v: usize = sc.next();
        a[i] = v - 1;
        c[a[i]] += 1;
        l[a[i]][i] = 1;
    }

    for i in 0..m {
        for j in 1..n {
            l[i][j] += l[i][j - 1];
        }
    }

    let mut dp = vec![INF; 1 << 20];
    dp[0] = 0;
    for msk in 0..1 << m {
        let mut done = 0;
        for i in 0..m {
            if msk & (1 << i) == 1 {
                done += c[i];
            }
        }
        for nxt in 0..m {
            if msk & (1 << nxt) == 1 {
                continue;
            }
            let mut tot = dp[msk];

            let mut nxt_cnt = l[nxt][done + c[nxt] - 1];
            if done > 0 {
                nxt_cnt -= l[nxt][done - 1];
            }
            tot += c[nxt] - nxt_cnt;

            dp[msk + (1 << nxt)] = min(dp[msk + (1 << nxt)], tot);
        }
    }

    println!("{}", dp[(1 << m) - 1]);
}
