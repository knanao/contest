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

fn main() {
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    let (h, w): (usize, usize) = (sc.next(), sc.next());
    let mut s = vec![vec!['#'; w]; h];
    let mut ddc = 0;
    for i in 0..h {
        let c = sc.chars();
        for j in 0..w {
            if c[j] == '#' {
                ddc += 1;
            }
            s[i][j] = c[j];
        }
    }

    let mut seen = vec![vec![false; w]; h];
    let dx = vec![0, 0, 1, -1];
    let dy = vec![1, -1, 0, 0];
    let mut mrc = 0;
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((0, 0, 0));

    while !q.is_empty() {
        let (x, y, c) = q.pop_front().unwrap();
        if seen[x][y] {
            continue;
        }
        seen[x][y] = true;

        if x == h - 1 && y == w - 1 {
            mrc = c;
            break;
        }

        for i in 0..4 {
            let (nx, ny) = (x as i32 + dx[i], y as i32 + dy[i]);
            if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                continue;
            }
            if seen[nx as usize][ny as usize] {
                continue;
            }
            if s[nx as usize][ny as usize] == '#' {
                continue;
            }
            q.push_back((nx as usize, ny as usize, c + 1));
        }
    }

    if mrc == 0 {
        println!("-1");
    } else {
        println!("{}", h * w - (mrc + ddc + 1));
    }
}
