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
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut min = std::i64::MAX;
    for i in 0..n {
        for j in 0..n {
            let (s, g) = (ab[i].0, ab[j].1);
            let d = ab.iter().fold(0i64, |sum, &(a, b)| {
                sum + (s - a).abs() + (a - b).abs() + (b - g).abs()
            });
            min = std::cmp::min(min, d);
        }
    }
    println!("{}", min);
}

// fn main() {
//     let cin = stdin();
//     let mut sc = Scanner::new(cin.lock());
//
//     let n: usize = sc.next();
//     let mut ab: Vec<(i64, i64)> = vec![(0, 0); n];
//     for i in 0..n {
//         ab[i] = (sc.next(), sc.next());
//     }
//
//     let mut ans = std::usize::MAX;
//     for i in 0..n {
//         for j in 0..n {
//             let mut c: i64 = 0;
//             let (s, g) = (ab[i].0, ab[j].1);
//             for k in 0..n {
//                 c += (s - ab[k].0).abs();
//                 c += ab[k].1 - ab[k].0;
//                 c += (g - ab[k].1).abs();
//             }
//             ans = std::cmp::min(ans, c as usize);
//         }
//     }
//     println!("{}", ans);
// }
