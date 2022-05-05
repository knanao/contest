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
        v: [(String, usize); n],
    }
    let card = [
        ("S".to_string(), 1),
        ("S".to_string(), 2),
        ("S".to_string(), 3),
        ("S".to_string(), 4),
        ("S".to_string(), 5),
        ("S".to_string(), 6),
        ("S".to_string(), 7),
        ("S".to_string(), 8),
        ("S".to_string(), 9),
        ("S".to_string(), 10),
        ("S".to_string(), 11),
        ("S".to_string(), 12),
        ("S".to_string(), 13),
        ("H".to_string(), 1),
        ("H".to_string(), 2),
        ("H".to_string(), 3),
        ("H".to_string(), 4),
        ("H".to_string(), 5),
        ("H".to_string(), 6),
        ("H".to_string(), 7),
        ("H".to_string(), 8),
        ("H".to_string(), 9),
        ("H".to_string(), 10),
        ("H".to_string(), 11),
        ("H".to_string(), 12),
        ("H".to_string(), 13),
        ("C".to_string(), 1),
        ("C".to_string(), 2),
        ("C".to_string(), 3),
        ("C".to_string(), 4),
        ("C".to_string(), 5),
        ("C".to_string(), 6),
        ("C".to_string(), 7),
        ("C".to_string(), 8),
        ("C".to_string(), 9),
        ("C".to_string(), 10),
        ("C".to_string(), 11),
        ("C".to_string(), 12),
        ("C".to_string(), 13),
        ("D".to_string(), 1),
        ("D".to_string(), 2),
        ("D".to_string(), 3),
        ("D".to_string(), 4),
        ("D".to_string(), 5),
        ("D".to_string(), 6),
        ("D".to_string(), 7),
        ("D".to_string(), 8),
        ("D".to_string(), 9),
        ("D".to_string(), 10),
        ("D".to_string(), 11),
        ("D".to_string(), 12),
        ("D".to_string(), 13),
    ];
    'card: for i in card.iter() {
        for j in &v {
            if i == j {
                continue 'card;
            }
        }
        let (x, y) = i;
        println!("{} {}", x, y);
    }
}
