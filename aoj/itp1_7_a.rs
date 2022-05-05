fn main() {
    let mut s: Vec<(i32, i32, i32)> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("");

        let mut mfr = buf.trim().split_whitespace();
        let (m, f, r): (i32, i32, i32) = (
            mfr.next().unwrap().parse().unwrap(),
            mfr.next().unwrap().parse().unwrap(),
            mfr.next().unwrap().parse().unwrap(),
        );
        match (m, f, r) {
            (-1, -1, -1) => break,
            _ => {
                s.push((m, f, r));
            }
        }
    }
    for i in s {
        let (m, f, r) = i;
        if m < 0 || f < 0 {
            println!("F");
            continue;
        }
        if m + f >= 80 {
            println!("A");
            continue;
        }
        if m + f >= 65 {
            println!("B");
            continue;
        }
        if m + f >= 50 || (m + f >= 30 && r >= 50) {
            println!("C");
            continue;
        }
        if m + f >= 30 {
            println!("D");
            continue;
        }
        println!("F");
    }
}
