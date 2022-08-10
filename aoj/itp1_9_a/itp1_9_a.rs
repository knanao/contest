fn main() {
    let mut w = String::new();
    std::io::stdin().read_line(&mut w).expect("");

    let mut c = 0;
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("");

        if s.trim() == "END_OF_TEXT" {
            break;
        }

        let t: Vec<&str> = s.trim().split_whitespace().collect();
        for i in t {
            if w.trim() == i.to_lowercase() {
                c += 1;
            }
        }
    }
    println!("{}", c);
}
