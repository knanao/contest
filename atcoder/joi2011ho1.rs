use std::cmp::{Ord, Ordering};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::io;
use std::io::{Read, Write};
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
            writeln!(io::stderr(), "Terminated with EOF").unwrap();
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

struct DisjointSet {
    numtree: usize,
    data: Vec<usize>,
    size: Vec<usize>,
}

#[allow(dead_code)]
impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            numtree: n,
            data: (0..n).map(|x| x).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.data[x] == x {
            x
        } else {
            let px = self.data[x];
            self.data[x] = self.find(px);
            self.data[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py {
            return;
        }
        if self.size[py] < self.size[px] {
            std::mem::swap(&mut px, &mut py);
        }
        self.data[px] = py;
        self.size[py] += self.size[px];
        self.numtree -= 1;
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        self.data[px] == self.data[py]
    }

    fn tree_size(&mut self, x: usize) -> usize {
        let px = self.find(self.data[x]);
        self.size[px]
    }
}

#[allow(dead_code)]
const INF: usize = 1 << 60;

// 2 2 3 6
// (3, 6) - (3, 1) - (1, 6) + (1, 1)
//
// 3 5 4 7
// (4, 7) - (4, 4) - (2, 7) + (2, 4)
//
// (c, d) - (c, b-1) - (a-1, d) + (a-1, b-1)
fn main() {
    let cin = io::stdin();
    let mut sc = Scanner::new(cin.lock());

    let (m, n, k): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut acc = vec![vec![(0, 0, 0); n + 1]; m + 1];
    for i in 1..=m {
        let c = sc.chars();
        for j in 1..=n {
            match c[j - 1] {
                'J' => acc[i][j].0 += 1,
                'O' => acc[i][j].1 += 1,
                'I' => acc[i][j].2 += 1,
                _ => continue,
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            acc[i + 1][j + 1].0 += acc[i][j + 1].0 + acc[i + 1][j].0 - acc[i][j].0;
            acc[i + 1][j + 1].1 += acc[i][j + 1].1 + acc[i + 1][j].1 - acc[i][j].1;
            acc[i + 1][j + 1].2 += acc[i][j + 1].2 + acc[i + 1][j].2 - acc[i][j].2;
        }
    }

    for _ in 0..k {
        let (a, b, c, d): (usize, usize, usize, usize) =
            (sc.next(), sc.next(), sc.next(), sc.next());
        let (j, o, i) = (
            acc[c][d].0 - acc[c][b - 1].0 - acc[a - 1][d].0 + acc[a - 1][b - 1].0,
            acc[c][d].1 - acc[c][b - 1].1 - acc[a - 1][d].1 + acc[a - 1][b - 1].1,
            acc[c][d].2 - acc[c][b - 1].2 - acc[a - 1][d].2 + acc[a - 1][b - 1].2,
        );
        println!("{} {} {}", j, o, i);
    }
}
