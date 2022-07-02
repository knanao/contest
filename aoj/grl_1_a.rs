use std::collections::{BinaryHeap, HashMap, VecDeque};
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

fn main() {
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    let (v, e, r): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut g: Vec<Vec<(usize, i32)>> = vec![vec![]; v];

    for _ in 0..e {
        let (s, t, d): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
        g[s].push((t, -(d as i32)));
    }

    for i in dijkstra(&g, r) {
        if i == std::i32::MIN {
            println!("INF");
        } else {
            println!("{}", -i);
        }
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
    let mut dist = vec![std::i32::MIN; graph.len()];
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
    return dist;
}
