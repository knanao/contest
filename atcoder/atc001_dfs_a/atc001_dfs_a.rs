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

    let (h, w): (i32, i32) = (sc.next(), sc.next());
    let field: Vec<String> = sc.vec(h as usize);

    let mut sh = 0;
    let mut sw = 0;
    let mut gh = 0;
    let mut gw = 0;
    for i in 0..h as usize {
        for j in 0..w as usize {
            let cs: Vec<char> = field[i].chars().collect();
            if cs[j] == 's' {
                sh = i as i32;
                sw = j as i32;
            }
            if cs[j] == 'g' {
                gh = i as i32;
                gw = j as i32;
            }
        }
    }

    let dx: Vec<i32> = vec![1, 0, 0, -1];
    let dy: Vec<i32> = vec![0, 1, -1, 0];
    let mut seen = vec![vec![false; w as usize]; h as usize];
    dfs(&mut seen, &field, &dx, &dy, sh, sw, h, w);

    println!(
        "{}",
        if seen[gh as usize][gw as usize] {
            "Yes"
        } else {
            "No"
        }
    );
}

fn dfs(
    seen: &mut Vec<Vec<bool>>,
    field: &Vec<String>,
    dx: &Vec<i32>,
    dy: &Vec<i32>,
    sh: i32,
    sw: i32,
    h: i32,
    w: i32,
) {
    seen[sh as usize][sw as usize] = true;

    for i in 0..4 {
        let nh = sh + dx[i];
        let nw = sw + dy[i];

        if nh < 0 || nh >= h || nw < 0 || nw >= w {
            continue;
        }
        let cs: Vec<char> = field[nh as usize].chars().collect();
        if cs[nw as usize] == '#' {
            continue;
        }
        if seen[nh as usize][nw as usize] {
            continue;
        }

        dfs(seen, field, dx, dy, nh, nw, h, w);
    }
}
