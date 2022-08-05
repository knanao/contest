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

    loop {
        let (w, h): (usize, usize) = (sc.next(), sc.next());
        if (w, h) == (0, 0) {
            return;
        }
        let mut c: Vec<Vec<usize>> = vec![vec![0; w]; h];
        for i in 0..h {
            for j in 0..w {
                c[i][j] = sc.next();
            }
        }

        let dx = vec![0, 1, 0, -1, 1, 1, -1, -1];
        let dy = vec![1, 0, -1, 0, 1, -1, -1, 1];
        let mut seen = vec![vec![false; w]; h];

        let mut ans = 0;
        for i in 0..h {
            for j in 0..w {
                if c[i][j] == 0 {
                    continue;
                }
                if seen[i][j] {
                    continue;
                }
                dfs(
                    &mut seen, &c, &dx, &dy, i as i32, j as i32, w as i32, h as i32,
                );
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}

fn dfs(
    seen: &mut Vec<Vec<bool>>,
    c: &Vec<Vec<usize>>,
    dx: &Vec<i32>,
    dy: &Vec<i32>,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) {
    seen[x as usize][y as usize] = true;
    if c[x as usize][y as usize] == 0 {
        return;
    }

    for i in 0..8 {
        let pos_x = x + dx[i];
        let pos_y = y + dy[i];
        if pos_x < 0 || pos_x >= h || pos_y < 0 || pos_y >= w {
            continue;
        }
        if seen[pos_x as usize][pos_y as usize] {
            continue;
        }
        dfs(seen, c, dx, dy, pos_x, pos_y, w, h);
    }
}
