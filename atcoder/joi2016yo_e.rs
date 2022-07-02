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

    let (n, m, k, s): (usize, usize, usize, usize) = (sc.next(), sc.next(), sc.next(), sc.next());
    let (p, q): (i32, i32) = (sc.next(), sc.next());
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut dst = vec![INF; n];
    for _ in 0..k {
        let v: usize = sc.next();
        que.push_back(v - 1);
        dst[v - 1] = 0;
    }

    let mut g: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for _ in 0..m {
        let (a, b): (usize, usize) = (sc.next(), sc.next());
        let mut ca = -p;
        let mut cb = -p;
        if b == n || b == 1 {
            cb = 0;
        }
        if a == n || a == 1 {
            ca = 0;
        }
        if que.contains(&(a - 1)) {
            ca = -100000;
        }
        if que.contains(&(b - 1)) {
            cb = -100000;
        }
        g[a - 1].push((b - 1, cb));
        g[b - 1].push((a - 1, ca));
    }

    while !que.is_empty() {
        let u = que.pop_front().unwrap();
        for (v, _) in &g[u] {
            if dst[*v] <= dst[u] + 1 {
                continue;
            }
            dst[*v] = dst[u] + 1;
            que.push_back(*v);
        }
    }

    println!("dst: {:?}", dst);
    let ans = dijkstra(&g, 0, dst, s, q, &que);
    println!("ans: {:?}", ans);
    println!("{}", -ans[n - 1]);
}

fn dijkstra(
    graph: &Vec<Vec<(usize, i32)>>,
    start: usize,
    dst: Vec<usize>,
    s: usize,
    q: i32,
    que: &VecDeque<usize>,
) -> Vec<i32> {
    let mut dist = vec![std::i32::MIN; graph.len()];
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push((0, start));
    while let Some((acc, pos)) = heap.pop() {
        if dist[pos] > acc {
            continue;
        }

        for &(d, cost) in &graph[pos] {
            let mut cst = cost;
            if cost == -100000 || que.contains(&pos) {
                continue;
            }
            if dst[d] <= s {
                cst = -q;
            }
            if acc + cst > dist[d] {
                dist[d] = acc + cst;
                heap.push((acc + cst, d));
            }
        }
    }
    return dist;
}
