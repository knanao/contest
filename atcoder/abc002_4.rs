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
    let m: usize = sc.next();
    let mut xy: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for _ in 0..m {
        let (x, y): (usize, usize) = (sc.next(), sc.next());
        xy[x - 1][y - 1] = 1;
        xy[y - 1][x - 1] = 1;
    }

    let mut ans = 0;
    for i in 0..1 << n {
        let mut g: Vec<usize> = vec![];
        for j in 0..n {
            if i >> j & 1 == 1 {
                g.push(j);
            }
        }
        let mut ok = true;
        for k in &g {
            for l in &g {
                if k == l {
                    continue;
                }
                if xy[*k][*l] != 1 {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            ans = std::cmp::max(ans, g.len())
        }
    }
    println!("{}", ans);
}
