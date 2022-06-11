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

    let (w, h): (usize, usize) = (sc.next(), sc.next());
    let mut v = vec![vec![0; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            v[i][j] = sc.next();
        }
    }

    let dx = vec![vec![0, 0, 1, 1, -1, -1], vec![0, 0, 1, 1, -1, -1]];
    let dy = vec![vec![-1, 1, -1, 0, 0, -1], vec![-1, 1, 0, 1, 0, 1]];
    let mut seen = vec![vec![false; w + 2]; h + 2];
    rec(&mut seen, &v, &dx, &dy, 0, 0, h, w);

    for i in 1..=h {
        for j in 1..=w {
            if !seen[i][j] {
                v[i][j] = 1;
            }
        }
    }

    let mut ans = 0;
    for i in 1..=h {
        for j in 1..=w {
            if v[i][j] == 0 {
                continue;
            }
            for k in 0..6 {
                let nx = i as i32 + dx[i % 2][k];
                let ny = j as i32 + dy[i % 2][k];
                if v[nx as usize][ny as usize] == 0 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

fn rec(
    seen: &mut Vec<Vec<bool>>,
    v: &Vec<Vec<usize>>,
    dx: &Vec<Vec<i32>>,
    dy: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    h: usize,
    w: usize,
) {
    seen[x][y] = true;
    for i in 0..6 {
        let nx = x as i32 + dx[x % 2][i];
        let ny = y as i32 + dy[x % 2][i];

        if nx < 0 || nx >= (h + 2) as i32 || ny < 0 || ny >= (w + 2) as i32 {
            continue;
        }
        if v[nx as usize][ny as usize] == 1 {
            continue;
        }
        if !seen[nx as usize][ny as usize] {
            rec(seen, v, dx, dy, nx as usize, ny as usize, h, w);
        }
    }
}
