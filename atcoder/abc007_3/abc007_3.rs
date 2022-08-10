use std::collections::VecDeque;
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

    let (r, c): (usize, usize) = (sc.next(), sc.next());
    let (sx, sy): (usize, usize) = (sc.next(), sc.next());
    let (gx, gy): (usize, usize) = (sc.next(), sc.next());
    let mut v: Vec<Vec<char>> = vec![vec![]; r];
    for i in 0..r {
        let cs: Vec<char> = sc.chars();
        for j in 0..c {
            v[i].push(cs[j]);
        }
    }

    let mut dst: Vec<Vec<i32>> = vec![vec![-1; c]; r];
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((sx - 1, sy - 1, 0));
    dst[sx - 1][sy - 1] = 0;

    let dx = vec![0, 1, -1, 0];
    let dy = vec![1, 0, 0, -1];

    while !q.is_empty() {
        let (x, y, cost) = q.pop_front().unwrap();
        if (x, y) == (gx - 1, gy - 1) {
            println!("{}", cost);
            return;
        }
        for i in 0..4 {
            let (vx, vy) = (x as i32 + dx[i], y as i32 + dy[i]);
            if vx > 0 && vx < r as i32 && vy > 0 && vy < c as i32 {
                if v[vx as usize][vy as usize] == '.' && dst[vx as usize][vy as usize] == -1 {
                    dst[vx as usize][vy as usize] = cost as i32 + 1;
                    q.push_back((vx as usize, vy as usize, cost + 1));
                }
            }
        }
    }
}
