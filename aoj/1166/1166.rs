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

    loop {
        let (w, h): (usize, usize) = (sc.next(), sc.next());
        if w == 0 && h == 0 {
            break;
        }

        let mut vertical: Vec<Vec<usize>> = vec![vec![]; h];
        let mut horizontal: Vec<Vec<usize>> = vec![vec![]; h - 1];

        for i in 0..h - 1 {
            vertical[i] = sc.vec(w - 1);
            horizontal[i] = sc.vec(w);
        }
        vertical[h - 1] = sc.vec(w - 1);

        let mut seen = vec![vec![false; w]; h];
        let mut ans = 0;
        let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
        q.push_front((0, 0, 0));

        let (nv, nh) = padding(&vertical, &horizontal, w);

        while let Some((x, y, c)) = q.pop_front() {
            seen[x][y] = true;
            if x == h - 1 && y == w - 1 {
                ans = c + 1;
            }

            let d = [
                (0, 1, nv[x][y + 1]),
                (0, -1, nv[x][y]),
                (1, 0, nh[x + 1][y]),
                (-1, 0, nh[x][y]),
            ];
            for &(dx, dy, dw) in &d {
                if dw != 0 {
                    continue;
                }
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                    continue;
                }
                if seen[nx as usize][ny as usize] {
                    continue;
                }
                q.push_back((nx as usize, ny as usize, c + 1));
            }
        }
        println!("{}", ans);
    }
}

fn padding(
    v: &Vec<Vec<usize>>,
    h: &Vec<Vec<usize>>,
    w: usize,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let nv: Vec<Vec<usize>> = v
        .iter()
        .map(|x| {
            std::iter::once(1)
                .chain(x.iter().cloned())
                .chain(std::iter::once(1))
                .collect()
        })
        .collect();

    let nh: Vec<Vec<usize>> = std::iter::once(vec![1; w])
        .chain(h.iter().cloned())
        .chain(std::iter::once(vec![1; w]))
        .collect();

    return (nv, nh);
}
