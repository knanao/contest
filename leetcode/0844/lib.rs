struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut s_v, mut t_v) = (Vec::new(), Vec::new());

        for c in s.chars() {
            if c == '#' {
                s_v.pop();
            } else {
                s_v.push(c);
            }
        }

        for c in t.chars() {
            if c == '#' {
                t_v.pop();
            } else {
                t_v.push(c);
            }
        }

        if s_v.len() != t_v.len() {
            return false;
        }

        let n = s_v.len();
        let mut ans = true;
        (0..n).enumerate().for_each(|(i, _)| {
            if s_v[i] != t_v[i] {
                ans = false;
            }
        });
        ans
    }
}
