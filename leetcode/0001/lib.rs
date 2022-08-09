use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            map.insert(v, i);
        }

        for (i, v) in nums.iter().enumerate() {
            let k: i32 = target - v;
            if map.contains_key(&k) && *map.get(&k).unwrap() != i {
                return vec![*map.get(&k).unwrap() as i32, i as i32];
            }
        }
        vec![]
    }
}
