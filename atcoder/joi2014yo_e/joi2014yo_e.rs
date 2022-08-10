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
    let mut cr: Vec<(usize, usize)> = vec![(0, 0); n];
    for i in 0..n {
        cr[i] = (sc.next(), sc.next());
    }
    let mut g: Vec<Vec<usize>> = vec![vec![0; n]; k];
    for _ in 0..k {
        let (a, b) = (sc.next::<usize>() - 1, sc.next::<usize>() - 1);
        g[a].push(b);
        g[b].push(a);
    }

    let mut h: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for i in 0..n {
        let (c, r) = cr[i];
        let mut seen = vec![false; n];
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        que.push_back((i, 0));
        seen[i] = true;
        while let Some((v, dist)) = que.pop_front() {
            for &j in &g[v] {
                if seen[j] {
                    continue;
                }
                h[i].push((j, -(c as i32)));
                seen[j] = true;
                if dist + 1 < r {
                    que.push_back((j, dist + 1));
                }
            }
        }
    }

    let ans = dijkstra(&mut h, 0);
    println!("{}", -ans[n - 1]);
}

fn dijkstra(g: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
    let mut dist = vec![std::i32::MIN; g.len()];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((0, start));
    while let Some((acc, pos)) = heap.pop() {
        if dist[pos] > acc {
            continue;
        }

        for &(dst, cost) in &g[pos] {
            if acc + cost > dist[dst] {
                dist[dst] = acc + cost;
                heap.push((acc + cost, dst));
            }
        }
    }
    return dist;
}
