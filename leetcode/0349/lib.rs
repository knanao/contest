use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret: HashSet<i32> = HashSet::new();
        for i in nums1 {
            if nums2.contains(&i) {
                ret.insert(i);
            }
        }
        ret.into_iter().collect()
    }
}
