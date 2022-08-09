use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for i in strs {
            let mut cs: Vec<char> = i.chars().collect();
            cs.sort();
            let s = String::from_iter(cs);
            if !map.contains_key(&s) {
                map.insert(s, vec![i]);
            } else {
                map.get_mut(&s).unwrap().push(i);
            }
        }

        map.into_values().collect()
    }
}
