macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        s: String,
    }
    let d = ["dream", "dreamer", "erase", "eraser"];
    let s_rev = s.chars().rev().collect::<String>();
    let d_rev = d
        .iter()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    let mut m = true;
    let mut i = 0;
    while i < s.len() {
        let mut m2 = false;
        for j in &d_rev {
            if i + j.len() > s_rev.len() {
                continue;
            }
            if &s_rev[i..i + j.len()] == *j {
                i += j.len();
                m2 = true;
            }
        }
        if !m2 {
            m = false;
            break;
        }
    }
    println!("{}", if m { "YES" } else { "NO" });
}
