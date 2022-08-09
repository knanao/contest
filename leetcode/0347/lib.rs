#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            if count.get(&i) != None {
                *count.get_mut(&i).unwrap() += 1;
            } else {
                count.insert(i, 1);
            }
        }

        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for (k, v) in count {
            heap.push((v, k));
        }

        let mut ret = vec![];
        for _ in 0..k {
            let (_, v) = heap.pop().unwrap();
            ret.push(v);
        }
        ret
    }
}
