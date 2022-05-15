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
        n: u64,
        s: [String; n],
    }

    let mut m: Vec<u64> = vec![0; 5];
    for i in s {
        let v = i.chars().next().unwrap();
        match v {
            'M' => m[0] += 1,
            'A' => m[1] += 1,
            'R' => m[2] += 1,
            'C' => m[3] += 1,
            'H' => m[4] += 1,
            _ => continue,
        }
    }

    let p = vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 2];
    let q = vec![1, 1, 1, 2, 2, 3, 2, 2, 3, 3];
    let r = vec![2, 3, 4, 3, 4, 4, 3, 4, 4, 4];
    let mut ret = 0;
    for i in 0..10 {
        ret += m[p[i]] * m[q[i]] * m[r[i]];
    }
    println!("{}", ret);
}
