fn main() {
    let mut v: Vec<(i32, i32)> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("");

        let mut nx = buf.trim().split_whitespace();
        let (n, x): (i32, i32) = (
            nx.next().unwrap().parse().unwrap(),
            nx.next().unwrap().parse().unwrap(),
        );
        match (n, x) {
            (0, 0) => break,
            _ => v.push((n, x)),
        }
    }

    for (n, x) in v {
        let mut count = 0;
        for i in 1..=n {
            for j in 1..i {
                for k in 1..j {
                    if i + j + k == x {
                        count += 1;
                    }
                }
            }
        }
        println!("{}", count)
    }
}
