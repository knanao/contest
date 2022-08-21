struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        Self::generate(&mut ans, &mut Vec::new(), 0, 0, n as usize);
        ans
    }

    fn generate(ans: &mut Vec<String>, v: &mut Vec<char>, s: usize, e: usize, max: usize) {
        if v.len() == 2 * max {
            ans.push(v.clone().into_iter().collect());
            return;
        }

        if s < max {
            v.push('(');
            Self::generate(ans, v, s + 1, e, max);
            v.pop();
        }

        if e < s {
            v.push(')');
            Self::generate(ans, v, s, e + 1, max);
            v.pop();
        }
    }
}
