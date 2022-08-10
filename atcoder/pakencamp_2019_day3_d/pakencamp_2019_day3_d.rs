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

    let n: usize = sc.next();
    let mut s = vec![vec![0; n]; 5];
    for i in 0..5 {
        let c = sc.chars();
        for j in 0..n {
            match c[j] {
                'R' => s[i][j] = 0,
                'B' => s[i][j] = 1,
                'W' => s[i][j] = 2,
                '#' => s[i][j] = 3,
                _ => continue,
            }
        }
    }

    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            let mut min = INF;
            for k in 0..3 {
                if j == k {
                    continue;
                }
                min = std::cmp::min(min, dp[i][k]);
            }
            dp[i + 1][j] += min + count(&s, i, j);
        }
    }
    let mut ans = INF;
    for i in 0..3 {
        ans = std::cmp::min(ans, dp[n][i]);
    }
    println!("{}", ans);
}

fn count(s: &Vec<Vec<usize>>, l: usize, color: usize) -> usize {
    let mut ret = 0;
    (0..5).for_each(|i| {
        if s[i][l] != color {
            ret += 1
        }
    });
    return ret;
}
