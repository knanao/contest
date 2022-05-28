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

struct DisjointSet {
    numtree: usize,
    data: Vec<usize>,
    size: Vec<usize>,
}

#[allow(dead_code)]
impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            numtree: n,
            data: (0..n).map(|x| x).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.data[x] == x {
            x
        } else {
            let px = self.data[x];
            self.data[x] = self.find(px);
            self.data[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py {
            return;
        }
        if self.size[py] < self.size[px] {
            std::mem::swap(&mut px, &mut py);
        }
        self.data[px] = py;
        self.size[py] += self.size[px];
        self.numtree -= 1;
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        self.data[px] == self.data[py]
    }

    fn tree_size(&mut self, x: usize) -> usize {
        let px = self.find(self.data[x]);
        self.size[px]
    }
}

fn main() {
    let cin = stdin();
    let mut sc = Scanner::new(cin.lock());

    let (v, e): (usize, usize) = (sc.next(), sc.next());
    let mut edge: Vec<(usize, usize, usize)> =
        (0..e).map(|_| (sc.next(), sc.next(), sc.next())).collect();

    let mut uf = DisjointSet::new(v);
    let mut tot = 0;
    edge.sort_by_key(|e| e.2);

    for &(sr, ds, w) in edge.iter() {
        if !uf.same(sr, ds) {
            uf.unite(sr, ds);
            tot += w;
        }
    }

    println!("{}", tot)
}
