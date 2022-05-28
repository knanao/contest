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

    let (v, e): (usize, usize) = (sc.next(), sc.next());
    let mut g = vec![vec![None; v]; v];

    for _ in 0..e {
        let (s, t, d): (usize, usize, i32) = (sc.next(), sc.next(), sc.next());
        g[s][t] = Some(d);
    }

    for i in 0..v {
        for j in 0..v {
            if i == j {
                g[i][j] = Some(0);
            }
        }
    }

    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if let (Some(a), Some(b)) = (g[i][k], g[k][j]) {
                    if g[i][j].is_none() || a + b < g[i][j].unwrap() {
                        g[i][j] = Some(a + b);
                    };
                }
            }
        }
    }

    g.iter().for_each(|x| {
        x.iter().for_each(|x| {
            if let Some(y) = x {
                if *y < 0 {
                    println!("NEGATIVE CYCLE");
                    return;
                }
            }
        })
    });
    for i in 0..v {
        for j in 0..v {
            match g[i][j] {
                Some(x) => print!("{}", x),
                None => print!("INF"),
            }
            if j != v - 1 {
                print!(" ");
            }
        }
        println!("");
    }
}
