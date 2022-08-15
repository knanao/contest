struct Solution;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..(s.len() + 1) {
            for w in word_dict.iter() {
                if w.len() > i {
                    continue;
                }

                let sub = &s[(i - w.len())..i];
                if dp[i - w.len()] && sub == w {
                    dp[i] = true;
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        println!("ex1");
        let mut input = (
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        );
        let mut got = Solution::word_break(input.0, input.1);
        assert!(got);

        println!("ex2");
        input = (
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()],
        );
        got = Solution::word_break(input.0, input.1);
        assert!(got);

        println!("ex3");
        input = (
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string(),
            ],
        );
        got = Solution::word_break(input.0, input.1);
        assert!(!got);

        println!("ex4");
        input = (
            "aaaaaaaa".to_string(),
            vec!["aaaa".to_string(), "aaa".to_string(), "aa".to_string()],
        );
        got = Solution::word_break(input.0, input.1);
        assert!(got);
    }
}
