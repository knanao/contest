use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut ret = HashSet::new();
        for i in emails {
            ret.insert(Solution::trim_special_letter(i));
        }
        ret.len() as i32
    }

    fn trim_special_letter(v: String) -> String {
        let mut s = 0;
        let mut e = 0;
        let cs: Vec<char> = v.chars().collect();
        let mut ret: Vec<char> = vec![];
        for (i, &c) in cs.iter().enumerate() {
            match c {
                '+' => {
                    if s == 0 && e == 0 {
                        s = i;
                    }
                    if s != 0 && e == 0 {
                        continue;
                    }
                }
                '@' => e = i,
                '.' => {
                    if e == 0 {
                        continue;
                    }
                }
                _ => {
                    if s != 0 && e == 0 {
                        continue;
                    }
                }
            }
            ret.push(c);
        }
        String::from_iter(ret)
    }
}
