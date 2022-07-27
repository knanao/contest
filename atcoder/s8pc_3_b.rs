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

    let (h, w, k): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut a = vec![vec![0usize; w]; h];
    for i in 0..h {
        let s: String = sc.next();
        let ss = s.as_bytes();
        for j in 0..w {
            a[i][j] = (ss[j] as char).to_string().parse::<usize>().unwrap();
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let mut blocks = a.clone();
            blocks[i][j] = 0;

            for y in (1..h).rev() {
                if blocks[y][j] == 0 {
                    let tmp = blocks[y][j];
                    blocks[y][j] = blocks[y - 1][j];
                    blocks[y - 1][j] = tmp;
                }
            }

            let mut ret = 0;
            let mut acc = 0u32;
            loop {
                let bs = delete(h, w, k, &mut blocks);
                if bs.len() == 0 {
                    break;
                }
                for (p, c) in bs {
                    ret += 2i32.pow(acc) as usize * p * c;
                }
                fall(h, w, &mut blocks);
                acc += 1;
            }
            ans = std::cmp::max(ans, ret);
        }
    }
    println!("{}", ans);
}

fn fall(h: usize, w: usize, blocks: &mut Vec<Vec<usize>>) {
    for i in 1..h {
        for y in (i..h).rev() {
            for x in 0..w {
                if blocks[y][x] == 0 {
                    let tmp = blocks[y][x];
                    blocks[y][x] = blocks[y - 1][x];
                    blocks[y - 1][x] = tmp;
                }
            }
        }
    }
}

fn delete(h: usize, w: usize, k: usize, blocks: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    for i in 0..h {
        for j in 0..w {
            if blocks[i][j] == 0 {
                continue;
            }
            let mut d = 1;
            while j + d < w && blocks[i][j] == blocks[i][j + d] {
                d += 1;
            }
            if d < k {
                continue;
            }
            ret.push((blocks[i][j], d));
            for x in 0..d {
                blocks[i][j + x] = 0;
            }
        }
    }
    ret
}
