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
    let mut xy: Vec<(i32, i32)> = vec![(0, 0); n];
    let mut nums = vec![0 as i32; n];
    for i in 0..n {
        xy[i] = (sc.next(), sc.next());
        nums[i] = (i + 1) as i32;
    }

    let mut c: f64 = 0.0;
    let mut s: f64 = 0.0;
    while {
        c += 1.0;
        for i in 0..n - 1 {
            let (x1, y1) = xy[(nums[i] - 1) as usize];
            let (x2, y2) = xy[(nums[i + 1] - 1) as usize];
            s += (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
        }
        next_permutation(&mut nums)
    } {}

    println!("{:.10}", s / c);
}

fn next_permutation(nums: &mut Vec<i32>) -> bool {
    use std::cmp::Ordering;
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| i32::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err(); // cannot fail because the binary search will never succeed
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();
    true
}
