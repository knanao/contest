struct Solution;
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s = s.into_bytes();
        words
            .into_iter()
            .map(|w| w.into_bytes())
            .filter(|t| Self::match_pattern(&s, &t))
            .count() as i32
    }

    fn match_pattern(s: &Vec<u8>, t: &Vec<u8>) -> bool {
        let (s_len, t_len) = (s.len(), t.len());
        let (mut i, mut j) = (0, 0);
        while i < s_len && j < t_len {
            let glob = s[i];
            if glob != t[j] {
                return false;
            }

            if i + 2 < s_len && glob == s[i + 1] && glob == s[i + 2] {
                while i < s_len && s[i] == glob {
                    i += 1;
                    if j < t_len && t[j] == glob {
                        j += 1;
                    }
                }
            } else {
                i += 1;
                j += 1;
            }
        }
        i == s_len && j == t_len
    }
}
