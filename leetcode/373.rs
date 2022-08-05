#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut sums = BinaryHeap::new();
        for (i, _) in nums1.iter().enumerate() {
            sums.push((-(nums1[i] + nums2[0]), i, 0));
        }

        let mut ret: Vec<Vec<i32>> = vec![];
        for _ in 0..k {
            let v = sums.pop();
            if v == None {
                return ret;
            }
            let (_, i, j) = v.unwrap();
            ret.push(vec![nums1[i], nums2[j]]);

            if j < nums2.len() - 1 {
                sums.push((-(nums1[i] + nums2[j + 1]), i, j + 1));
            }
        }
        ret
    }
}
