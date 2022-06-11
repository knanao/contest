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

    let (m, n): (usize, usize) = (sc.next(), sc.next());
    let mut t = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            t[i][j] = sc.next();
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if t[i][j] != 1 {
                continue;
            }
            let mut seen = vec![vec![false; m]; n];
            let r = dfs(&mut seen, &t, i as i32, j as i32, m as i32, n as i32);
            ans = std::cmp::max(ans, r);
        }
    }
    println!("{}", ans);
}

fn dfs(seen: &mut Vec<Vec<bool>>, t: &Vec<Vec<usize>>, x: i32, y: i32, m: i32, n: i32) -> usize {
    seen[x as usize][y as usize] = true;

    let dx = vec![0, 1, -1, 0];
    let dy = vec![1, 0, 0, -1];

    let mut ans = 1;
    for i in 0..4 {
        let (px, py) = (x + dx[i], y + dy[i]);
        if x + dx[i] < 0 || x + dx[i] >= n || y + dy[i] < 0 || y + dy[i] >= m {
            continue;
        }
        if seen[px as usize][py as usize] {
            continue;
        }
        if t[px as usize][py as usize] != 1 {
            continue;
        }
        let r = dfs(seen, t, x + dx[i], y + dy[i], m, n) + 1;
        ans = std::cmp::max(ans, r);
    }
    ans
}
