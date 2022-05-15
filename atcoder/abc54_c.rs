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

    let (n, m): (usize, usize) = (sc.next(), sc.next());
    let mut a: Vec<i32> = vec![0; m];
    let mut b: Vec<i32> = vec![0; m];

    for i in 0..m {
        a[i] = sc.next();
        b[i] = sc.next();
    }

    let mut nums: Vec<i32> = (1 as i32..=n as i32).collect();
    let mut c: u64 = 0;
    while {
        if nums[0] == 1 {
            let ok = nums.windows(2).all(|w| {
                for i in 0..m {
                    if w[0] == a[i] && w[1] == b[i] {
                        return true;
                    }
                    if w[0] == b[i] && w[1] == a[i] {
                        return true;
                    }
                }
                false
            });
            if ok {
                c += 1;
            }
        }

        next_permutation(&mut nums)
    } {}

    println!("{}", c);
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
