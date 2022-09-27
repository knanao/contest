use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    // pub fn length_of_longest_substring(s: String) -> i32 {
    //     let cs: Vec<char> = s.chars().collect();
    //     let mut ret = 0;
    //     for i in 0..s.len() {
    //         let mut map = HashSet::new();
    //         let mut count = 0;
    //         for j in i..s.len() {
    //             if map.get(&cs[j]).is_some() {
    //                 break;
    //             }
    //             map.insert(cs[j]);
    //             count += 1;
    //         }
    //         ret = std::cmp::max(ret, count);
    //     }
    //     ret
    // }
    pub fn length_of_longest_substring(s: String) -> i32 {
        let cs: Vec<char> = s.chars().collect();
        let mut sub: HashMap<char, usize> = HashMap::new();
        let (mut ret, mut i) = (0, 0);
        for j in 0..s.len() {
            if let Some(v) = sub.get_mut(&cs[j]) {
                *v += 1;
            } else {
                sub.insert(cs[j], 1);
            }
            while *sub.get(&cs[j]).unwrap() > 1 {
                sub.insert(cs[i], *sub.get(&cs[i]).unwrap() - 1);
                i += 1;
            }
            ret = std::cmp::max(ret, j - i + 1);
        }
        ret as i32
    }
}
