use std::collections::BTreeMap;

struct Solution;
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let mut ret = 1;
        let mut odd = vec![false; arr.len()];
        let mut even = vec![false; arr.len()];
        odd[arr.len() - 1] = true;
        even[arr.len() - 1] = true;

        let mut map = BTreeMap::new();
        map.insert(*arr.last().unwrap(), arr.len() - 1);

        for i in (0..arr.len() - 1).rev() {
            let from = arr[i];

            if let Some((_, &idx)) = map.range(from..).next() {
                odd[i] = even[idx];
            }

            if let Some((_, &idx)) = map.range(..=from).next_back() {
                even[i] = odd[idx];
            }

            if odd[i] {
                ret += 1;
            }
            map.insert(from, i);
        }
        ret
    }
}
