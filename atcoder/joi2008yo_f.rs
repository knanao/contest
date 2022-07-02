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

#[allow(dead_code)]
const INF: usize = 1 << 60;

fn main() {
    let cin = io::stdin();
    let mut sc = Scanner::new(cin.lock());

    let (n, k): (usize, usize) = (sc.next(), sc.next());
    let mut g: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

    let mut ans: Vec<i32> = vec![];
    for _ in 0..k {
        let o: usize = sc.next();
        if o == 1 {
            let (c, d, e): (usize, usize, i32) = (sc.next(), sc.next(), sc.next());
            g[c - 1].push((d - 1, -(e as i32)));
            g[d - 1].push((c - 1, -(e as i32)));
        } else {
            let (a, b): (usize, usize) = (sc.next(), sc.next());
            ans.push(dijkstra(&mut g, a - 1, b - 1));
        }
    }

    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize, end: usize) -> i32 {
    let mut dist = vec![std::i32::MIN; 100];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((0, start));
    while let Some((acc, pos)) = heap.pop() {
        if dist[pos] > acc {
            continue;
        }

        for &(dst, cost) in &graph[pos] {
            if acc + cost > dist[dst] {
                dist[dst] = acc + cost;
                heap.push((acc + cost, dst));
            }
        }
    }

    if dist[end] == std::i32::MIN {
        return -1;
    } else {
        return -dist[end];
    }
}
