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

fn main() {
    let cin = io::stdin();
    let mut sc = Scanner::new(cin.lock());

    let n: usize = sc.next();
    let mut exists = false;
    let mut kk = 0;
    for k in 1..=n {
        if k * (k + 1) / 2 == n {
            exists = true;
            kk = k;
            break;
        }
    }

    if !exists {
        println!("No");
    }

    println!("Yes");
    println!("{}", kk + 1);
    let mut res = vec![vec![]; kk + 1];
    let mut sum = 1;
    for k in (1..=kk).rev() {
        for i in 0..k {
            res[kk - k].push(sum + i);
            res[kk - k + i + 1].push(sum + i);
        }
        sum += k;
    }

    for i in 0..kk + 1 {
        print!("{} ", res[i].len());
        for j in 0..res[i].len() {
            if j != res[i].len() {
                print!("{} ", res[i][j]);
            } else {
                print!("{}", res[i][j]);
            }
        }
        println!();
    }
}
