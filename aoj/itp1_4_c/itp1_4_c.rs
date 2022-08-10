fn main() {
    let mut v: Vec<i32> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        let mut aopb = buf.trim().splitn(3, " ");
        let (a, op, b): (i32, &str, i32) = (
            aopb.next().unwrap().parse().unwrap(),
            aopb.next().unwrap(),
            aopb.next().unwrap().parse().unwrap(),
        );
        match op.as_ref() {
            "+" => v.push(a + b),
            "-" => v.push(a - b),
            "*" => v.push(a * b),
            "/" => v.push(a / b),
            _ => break,
        }
    }
    for i in v {
        println!("{}", i);
    }
}
