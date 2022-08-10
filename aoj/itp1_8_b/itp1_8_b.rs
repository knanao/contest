fn main() {
    let mut ret: Vec<u32> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("");

        let mut sum: u32 = 0;
        for x in buf.trim().as_bytes() {
            sum += (x - b'0') as u32;
        }

        if sum > 0 {
            ret.push(sum);
        } else {
            break;
        }
    }
    ret.iter().for_each(|i| println!("{}", i));
}
