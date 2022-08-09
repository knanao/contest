use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let l = begin_word.len();
        let mut dict: HashMap<String, Vec<String>> = HashMap::new();

        word_list.iter().for_each(|w| {
            for i in 0..l {
                let s = w.as_str();
                let new = format!("{}*{}", &s[0..i], &s[i + 1..l]);
                if let Some(v) = dict.get_mut(&new) {
                    v.push(s.to_string());
                } else {
                    dict.insert(new, vec![s.to_string()]);
                }
            }
        });

        let mut set: HashSet<String> = HashSet::new();
        let mut graph: VecDeque<(String, i32)> = VecDeque::new();
        set.insert(begin_word.to_string());
        graph.push_back((begin_word, 1));

        while let Some(v) = graph.pop_front() {
            let s = v.0.as_str();
            for i in 0..s.len() {
                let key = (&s[0..i]).to_string() + "*" + &s[i + 1..];
                if let Some(vs) = dict.get(&key) {
                    for next in vs.iter() {
                        if *next == end_word {
                            return v.1 + 1;
                        }
                        if !set.contains(next) {
                            set.insert(next.to_string());
                            graph.push_back((next.to_string(), v.1 + 1));
                        }
                    }
                }
            }
        }
        0
    }
}
