struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() == 1 {
            return s;
        }

        if num_rows < 2 {
            return s.into();
        }

        let mut ret = vec![String::new(); num_rows as usize];
        let (mut i, mut flag) = (0, true);
        for c in s.chars() {
            ret[i].push(c);
            i = if flag { i + 1 } else { i - 1 };
            flag = flag == (i > 0 && i < num_rows as usize - 1);
        }

        ret.concat()
    }
}
