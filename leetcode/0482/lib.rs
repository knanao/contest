struct Solution;
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let c = s.replace("-", "");
        let c_len = c.len() as i32;
        if c_len == 0 {
            return String::from("");
        }

        let (mut sec, mut f) = (c_len / k - 1, k);
        if c_len % k != 0 {
            f = c_len % k;
            sec += 1;
        }

        let mut ret: Vec<String> = vec![];
        let mut idx = f as usize;
        let cs = c.chars().collect::<Vec<char>>();
        ret.push(cs[..idx].iter().collect());

        for _ in 0..sec {
            ret.push(cs[idx..idx + k as usize].iter().collect());
            idx = idx + k as usize;
        }
        ret.iter()
            .map(|x| x.to_string().to_ascii_uppercase())
            .collect::<Vec<String>>()
            .join("-")
    }
}
