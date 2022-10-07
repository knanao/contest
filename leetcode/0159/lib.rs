use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        if s.len() < 3 {
            return s.len() as i32;
        }
        let cs: Vec<char> = s.chars().collect();
        let mut map = HashMap::new();
        let mut ans = 2;
        let (mut left, mut right) = (0, 0);
        while right < s.len() {
            map.insert(cs[right], right);
            right += 1;

            if map.len() == 3 {
                let mut delete = std::usize::MAX;
                for &v in map.values() {
                    delete = std::cmp::min(delete, v);
                }
                map.remove(&cs[delete]);
                left = delete + 1;
            }
            ans = std::cmp::max(ans, right - left);
        }
        ans as i32
    }
}
