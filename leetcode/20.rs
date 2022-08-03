struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v: Vec<char> = vec![];
        for c in s.chars() {
            match c {
                '(' => v.push(')'),
                '{' => v.push('}'),
                '[' => v.push(']'),
                ')' | '}' | ']' if Some(c) != v.pop() => return false,
                _ => continue,
            }
        }
        v.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let input = String::from("([)]");
        let got = Solution::is_valid(input);
        assert_eq!(got, true);
    }
}
