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

    let n: usize = sc.next();
    let mut mp = vec![vec!['.'; 8]; 8];
    let mut path = vec![vec![0; 8]; 8];

    for _ in 0..n {
        let (x, y): (usize, usize) = (sc.next(), sc.next());
        mp[x][y] = 'Q';
        put(x, y, &mut path);
    }

    solve(&mut mp, &mut path, n);

    for i in 0..8 {
        for j in 0..8 {
            print!("{}", mp[i][j]);
        }
        println!("");
    }
}

fn solve(mp: &mut Vec<Vec<char>>, path: &mut Vec<Vec<usize>>, k: usize) -> bool {
    if k == 8 {
        return true;
    }

    for i in 0..8 {
        for j in 0..8 {
            if path[i][j] == 0 {
                put(i, j, path);
                mp[i][j] = 'Q';
                if solve(mp, path, k + 1) {
                    return true;
                }
                mp[i][j] = '.';
                delete(i, j, path);
            }
        }
    }

    return false;
}

fn put(x: usize, y: usize, path: &mut Vec<Vec<usize>>) {
    let dx = vec![1, 0, -1, 0, 1, 1, -1, -1];
    let dy = vec![0, 1, 0, -1, 1, -1, 1, -1];

    for i in 0..8 {
        let mut r = x as i32 + dx[i];
        let mut c = y as i32 + dy[i];

        while (0..8).contains(&r) && (0..8).contains(&c) {
            path[r as usize][c as usize] += 1;
            r += dx[i];
            c += dy[i];
        }
    }

    path[x][y] += 1;
}

fn delete(x: usize, y: usize, path: &mut Vec<Vec<usize>>) {
    let dx = vec![1, 0, -1, 0, 1, 1, -1, -1];
    let dy = vec![0, 1, 0, -1, 1, -1, 1, -1];

    for i in 0..8 {
        let mut r = x as i32 + dx[i];
        let mut c = y as i32 + dy[i];

        while (0..8).contains(&r) && (0..8).contains(&c) {
            path[r as usize][c as usize] -= 1;
            r += dx[i];
            c += dy[i];
        }
    }

    path[x][y] -= 1;
}
