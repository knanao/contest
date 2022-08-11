use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut c: HashMap<char, usize> = HashMap::new();
        let cs = s.chars().collect::<Vec<char>>();
        for &i in &cs {
            let v = c.get_mut(&i);
            if v != None {
                *v.unwrap() += 1;
            } else {
                c.insert(i, 1);
            }
        }

        for (i, v) in cs.iter().enumerate() {
            if *c.get(&v).unwrap() == 1 {
                return i as i32;
            }
        }
        -1
    }
}
