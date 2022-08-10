fn main() {
    let mut v: Vec<Vec<i32>> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        let mut xy = buf.trim().splitn(2, " ");
        let (x, y) = (
            xy.next().unwrap().parse().unwrap(),
            xy.next().unwrap().parse().unwrap(),
        );
        match (x, y) {
            (0, 0) => break,
            _ => {
                if x < y {
                    v.push(vec![x, y]);
                } else {
                    v.push(vec![y, x]);
                }
            }
        }
    }
    v.iter().for_each(|i| println!("{} {}", i[0], i[1]))
}
