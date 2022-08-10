fn main() {
    let mut v: Vec<i32> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        let x: i32 = buf.trim().parse().unwrap();
        if x == 0 {
            break;
        }
        v.push(x);
    }
    v.iter()
        .enumerate()
        .for_each(|(i, v)| println!("Case {}: {}", i + 1, v));
}
