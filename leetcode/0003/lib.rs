use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let cs: Vec<char> = s.chars().collect();
        let mut ret = 0;
        for i in 0..s.len() {
            let mut map = HashSet::new();
            let mut count = 0;
            for j in i..s.len() {
                if map.get(&cs[j]).is_some() {
                    break;
                }
                map.insert(cs[j]);
                count += 1;
            }
            ret = std::cmp::max(ret, count);
        }
        ret
    }
}
