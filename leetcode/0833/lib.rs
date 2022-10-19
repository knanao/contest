use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_replace_string(
        mut s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut m = HashMap::new();
        for i in 0..indices.len() {
            m.insert(indices[i] as usize, vec![&sources[i], &targets[i]]);
        }

        let n = s.len();
        for i in (0..n).rev() {
            if let Some(v) = m.get(&i) {
                let h = &s[i..(i + v[0].len())];
                if h == v[0] {
                    s.replace_range(i..(i + v[0].len()), v[1]);
                }
            }
        }
        s
    }
}
