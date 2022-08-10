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

    let (h, w, n): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut m: Vec<Vec<char>> = vec![vec![]; w];
    let mut sx = 0;
    let mut sy = 0;
    for i in 0..h {
        let cs: Vec<char> = sc.chars();
        for j in 0..w {
            if cs[j] == 'S' {
                sx = i;
                sy = j;
            }
            m[i].push(cs[j]);
        }
    }

    let mut ans = 0;
    let mut target = 1;
    let mut cost = 0;
    let mut dst = vec![vec![-1; w]; h];
    let mut q: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    q.push_back((sx, sy, cost, target));
    dst[sx][sy] = 1;

    let dx = vec![0, 1, -1, 0];
    let dy = vec![1, 0, 0, -1];

    while !q.is_empty() {
        if target > n {
            break;
        }
        let (x, y, c, t) = q.pop_front().unwrap();
        if m[x][y] != 'S' && m[x][y] != '.' && (m[x][y] as usize) - ('0' as usize) == t {
            dst = vec![vec![-1; w]; h];
            dst[x][y] = 1;
            ans += c;
            cost = 0;
            target = t + 1;
            q.clear();
        } else {
            cost = c;
        }

        for i in 0..4 {
            let (xi, yi) = (x as i32 + dx[i], y as i32 + dy[i]);
            if xi >= 0 && xi < h as i32 && yi >= 0 && yi < w as i32 {
                if dst[xi as usize][yi as usize] != -1 {
                    continue;
                }
                if m[xi as usize][yi as usize] != 'X' && m[xi as usize][yi as usize] != 'S' {
                    dst[xi as usize][yi as usize] = 1;
                    q.push_back((xi as usize, yi as usize, cost + 1, target));
                }
            }
        }
    }
    println!("{}", ans);
}
