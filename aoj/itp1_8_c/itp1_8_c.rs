fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");

    for c in b'a'..=b'z' {
        let mut cnt: u32 = 0;

        for &x in s.as_bytes() {
            if c == x || c == (x ^ 32) {
                cnt += 1;
            }
        }

        println!("{} : {}", c as char, cnt);
    }
}
