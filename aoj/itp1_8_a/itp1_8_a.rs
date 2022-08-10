fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");
    let ret = buf
        .as_bytes()
        .iter()
        .map(|&c| {
            if b'A' <= c && c <= b'Z' {
                (c - b'A' + b'a') as char
            } else if b'a' <= c && c <= b'z' {
                (c - b'a' + b'A') as char
            } else {
                c as char
            }
        })
        .collect::<String>();
    print!("{}", ret);
}
