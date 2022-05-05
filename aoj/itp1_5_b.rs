fn main() {
    let mut v: Vec<Vec<usize>> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        let mut iter = buf.trim().split_whitespace();
        let (h, w): (usize, usize) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );

        match (h, w) {
            (0, 0) => break,
            _ => v.push(vec![h, w]),
        }
    }
    for i in v {
        let (h, w) = (i[0], i[1]);
        for j in 0..h {
            for k in 0..w {
                if (j == 0 || j == h - 1) || (k == 0 || k == w - 1) {
                    print!("#");
                    continue;
                }
                print!(".");
            }
            println!();
        }
        println!();
    }
}
