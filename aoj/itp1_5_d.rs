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
        n: u32,
    }
    let mut v: Vec<u32> = Vec::new();
    for i in 3..=n {
        if i % 3 == 0 {
            v.push(i);
            continue;
        }
        let mut x = i;
        loop {
            if !is_included_three(x) {
                break;
            }
            if x % 10 == 3 {
                v.push(i);
                break;
            }
            x /= 10;
        }
    }

    let dst: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", dst.join(" "));
}

fn is_included_three(v: u32) -> bool {
    let mut i: u32 = v;
    loop {
        if i % 10 == 3 {
            return true;
        }
        if i / 10 == 0 {
            return false;
        }
        i /= 10;
    }
}
