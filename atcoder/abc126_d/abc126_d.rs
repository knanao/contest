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
    let mut g: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let (v, u, w): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
        g[v - 1].push((u - 1, w));
        g[u - 1].push((v - 1, w))
    }

    let mut colors: Vec<i32> = vec![-1; n];
    dfs(&mut colors, &g, 0, 0);
    colors.iter().for_each(|x| println!("{}", x));
}

fn dfs(colors: &mut Vec<i32>, g: &Vec<Vec<(usize, usize)>>, color: i32, z: usize) {
    colors[z] = color;

    for i in g[z].iter().clone() {
        let &(v, w) = i;
        if colors[v] != -1 {
            continue;
        }
        if w % 2 == 0 {
            dfs(colors, g, color, v);
        } else {
            dfs(colors, g, 1 - color, v);
        }
    }
}
