struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut next, mut count) = (0, 0);
        let cs: Vec<char> = t.chars().collect();
        for c in s.chars() {
            for i in next..t.len() {
                if c != cs[i] {
                    continue;
                }
                next = i + 1;
                count += 1;
                break;
            }
        }
        count == s.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        let mut input = ("aaaaaa".to_string(), "bbaaaa".to_string());
        let mut got = Solution::is_subsequence(input.0, input.1);
        assert_eq!(false, got);
    }
}
