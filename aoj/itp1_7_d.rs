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
    let l: usize = sc.next();

    let a: Vec<Vec<u64>> = (0..n).map(|_| sc.vec(m)).collect();
    let b: Vec<Vec<u64>> = (0..m).map(|_| sc.vec(l)).collect();

    for i in 0..n {
        for j in 0..l {
            let mut c: u64 = 0;
            for k in 0..m {
                c += a[i][k] * b[k][j];
            }
            print!("{}{}", c, if j == l - 1 { '\n' } else { ' ' });
        }
    }
    // input! {
    //     n: usize,
    //     m: usize,
    //     l: usize,
    //     a: [[i32; m]; n],
    //     b: [[i32; l]; m],
    // }
    // let mut v = vec![vec![0; l]; n];
    // for i in 0..n {
    //     for j in 0..l {
    //         for k in 0..m {
    //             v[i][j] += a[i][k] * b[k][j];
    //         }
    //     }
    // }
    // for i in v {
    //     let dst: Vec<String> = i.iter().map(|x| x.to_string()).collect();
    //     println!("{}", dst.join(" "));
    // }
}
